/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

#![allow(deprecated)]

use crate::{
    filter_system_labels, normalize_duration_from_ns, sanitize_service_name,
    security::filter_sensitive_env_vars, AutoComposeError, Deploy, HealthCheck, Logging,
    NetworkConfig, Placement, ResourceLimits, Resources, Result, Service, ServiceNetworks,
    UlimitConfig,
};
use bollard::container::{InspectContainerOptions, ListContainersOptions};
use bollard::models::{ContainerInspectResponse, ContainerSummary, RestartPolicyNameEnum};
use bollard::Docker;
use futures::stream::{FuturesUnordered, StreamExt};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct DockerProcessor {
    docker: Docker,
}

pub struct ProcessingOptions {
    pub include_sensitive: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct DockerContext {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Metadata")]
    metadata: HashMap<String, serde_json::Value>,
    #[serde(rename = "Endpoints")]
    endpoints: HashMap<String, EndpointConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EndpointConfig {
    #[serde(rename = "Host")]
    host: Option<String>,
    #[serde(rename = "SkipTLSVerify")]
    skip_tls_verify: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct DockerConfig {
    #[serde(rename = "currentContext")]
    current_context: Option<String>,
}

impl DockerProcessor {
    pub fn new() -> Result<Self> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(Self { docker })
    }
    
    pub fn new_with_host(docker_host: &str) -> Result<Self> {
        let docker = Docker::connect_with_http(docker_host, 120, bollard::API_DEFAULT_VERSION)?;
        Ok(Self { docker })
    }
    
    pub fn new_with_context(context_name: &str) -> Result<Self> {
        // Try to read Docker context configuration
        if let Some(context_endpoint) = Self::read_docker_context(context_name)? {
            let docker = Docker::connect_with_http(&context_endpoint, 120, bollard::API_DEFAULT_VERSION)?;
            Ok(Self { docker })
        } else {
            // Fallback to default if context not found
            eprintln!("Warning: Docker context '{}' not found, using default connection", context_name);
            Self::new()
        }
    }
    
    fn read_docker_context(context_name: &str) -> Result<Option<String>> {
        // First, try to read from Docker CLI config directory
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        let docker_config_path = PathBuf::from(&home).join(".docker");
        
        // Check if we should use the current context
        let target_context = if context_name == "current" {
            // Read the current context from config.json
            let config_file = docker_config_path.join("config.json");
            if config_file.exists() {
                let config_data = std::fs::read_to_string(&config_file)?;
                let config: DockerConfig = serde_json::from_str(&config_data)
                    .unwrap_or(DockerConfig { current_context: None });
                config.current_context.unwrap_or_else(|| "default".to_string())
            } else {
                "default".to_string()
            }
        } else {
            context_name.to_string()
        };
        
        // Special case for default context
        if target_context == "default" {
            // Check DOCKER_HOST environment variable first
            if let Ok(docker_host) = std::env::var("DOCKER_HOST") {
                return Ok(Some(docker_host));
            }
            // Otherwise use local socket
            return Ok(None);
        }
        
        // Try to read context metadata
        let contexts_dir = docker_config_path.join("contexts").join("meta");
        let context_hash = Self::hash_context_name(&target_context);
        let context_meta_file = contexts_dir.join(&context_hash).join("meta.json");
        
        if context_meta_file.exists() {
            let meta_data = std::fs::read_to_string(&context_meta_file)?;
            let context: DockerContext = serde_json::from_str(&meta_data)?;
            
            // Extract Docker endpoint
            if let Some(docker_endpoint) = context.endpoints.get("docker") {
                if let Some(host) = &docker_endpoint.host {
                    return Ok(Some(host.clone()));
                }
            }
        }
        
        Ok(None)
    }
    
    fn hash_context_name(name: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub async fn list_containers(&self, running_only: bool) -> Result<Vec<ContainerSummary>> {
        let list_options = ListContainersOptions::<String> {
            all: !running_only,
            ..Default::default()
        };

        Ok(self.docker.list_containers(Some(list_options)).await?)
    }

    pub async fn process_containers_parallel(
        &self,
        containers: Vec<ContainerSummary>,
    ) -> Result<(
        HashMap<String, Service>,
        HashMap<String, Value>,
        HashMap<String, Value>,
    )> {
        self.process_containers_parallel_with_options(containers, ProcessingOptions { include_sensitive: false }).await
    }
    
    pub async fn process_containers_parallel_with_options(
        &self,
        containers: Vec<ContainerSummary>,
        options: ProcessingOptions,
    ) -> Result<(
        HashMap<String, Service>,
        HashMap<String, Value>,
        HashMap<String, Value>,
    )> {
        let mut tasks = FuturesUnordered::new();

        for container in containers {
            let docker_clone = self.docker.clone();
            let include_sensitive = options.include_sensitive;
            tasks.push(tokio::spawn(async move {
                Self::process_single_container(docker_clone, container, include_sensitive).await
            }));
        }

        let mut services = HashMap::new();
        let mut networks = HashMap::new();
        let mut volumes = HashMap::new();

        while let Some(task_result) = tasks.next().await {
            match task_result {
                Ok(Ok((service_name, service, nets, vols))) => {
                    services.insert(service_name, service);
                    for net in nets {
                        networks
                            .entry(net)
                            .or_insert_with(|| Value::Mapping(serde_yaml::Mapping::new()));
                    }
                    for vol in vols {
                        volumes
                            .entry(vol)
                            .or_insert_with(|| Value::Mapping(serde_yaml::Mapping::new()));
                    }
                }
                Ok(Err(e)) => eprintln!("Error processing container: {:?}", e),
                Err(e) => eprintln!("Task failed: {:?}", e),
            }
        }

        Ok((services, networks, volumes))
    }

    async fn process_single_container(
        docker: Docker,
        container: ContainerSummary,
        include_sensitive_vars: bool,
    ) -> Result<(String, Service, Vec<String>, Vec<String>)> {
        let container_id = container.id.clone().ok_or_else(|| {
            AutoComposeError::ContainerInspection("Container ID is missing".to_string())
        })?;

        let inspect = docker
            .inspect_container(&container_id, None::<InspectContainerOptions>)
            .await?;

        Self::extract_service_from_inspect(inspect, container, include_sensitive_vars)
    }

    fn extract_service_from_inspect(
        inspect: ContainerInspectResponse,
        container: ContainerSummary,
        include_sensitive_vars: bool,
    ) -> Result<(String, Service, Vec<String>, Vec<String>)> {
        let (config, host_config, network_settings) = match (
            inspect.config,
            inspect.host_config,
            inspect.network_settings,
        ) {
            (Some(config), Some(host_config), Some(network_settings)) => {
                (config, host_config, network_settings)
            }
            _ => {
                return Err(AutoComposeError::ContainerInspection(
                    "Incomplete container information".to_string(),
                ))
            }
        };

        let image = config.image.unwrap_or_default();

        let container_name = container
            .names
            .unwrap_or_default()
            .first()
            .map(|name| sanitize_service_name(name));

        let hostname = config.hostname;

        let environment = config.env.and_then(|env_vars| {
            let env_map: HashMap<String, String> = env_vars
                .into_iter()
                .filter_map(|e| {
                    let parts: Vec<&str> = e.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        Some((parts[0].to_string(), parts[1].to_string()))
                    } else {
                        None
                    }
                })
                .collect();
            // Filter out sensitive environment variables unless include_sensitive is true
            let filtered_env = if include_sensitive_vars {
                env_map
            } else {
                filter_sensitive_env_vars(env_map)
            };
            if filtered_env.is_empty() {
                None
            } else {
                Some(filtered_env)
            }
        });

        let ports = network_settings.ports.as_ref().and_then(|port_map| {
            let port_list: Vec<String> = port_map
                .iter()
                .filter_map(|(port, bindings)| {
                    bindings.as_ref().map(|binds| {
                        binds
                            .iter()
                            .filter_map(|binding| {
                                let host_ip = binding.host_ip.as_deref().unwrap_or("0.0.0.0");
                                let host_port = binding.host_port.as_deref().unwrap_or_default();
                                // Skip IPv6 any-address bindings (::) as they're typically duplicates
                                if host_ip == "::" || host_ip.is_empty() || host_port.is_empty() {
                                    None
                                } else if host_ip == "0.0.0.0" {
                                    Some(format!("{}:{}", host_port, port))
                                } else {
                                    Some(format!("{}:{}:{}", host_ip, host_port, port))
                                }
                            })
                            .collect::<Vec<String>>()
                    })
                })
                .flatten()
                .collect();
            if port_list.is_empty() {
                None
            } else {
                Some(port_list)
            }
        });

        let volumes = host_config.binds.clone();
        let mut volume_defs = Vec::new();
        if let Some(binds) = &host_config.binds {
            for bind in binds {
                let parts: Vec<&str> = bind.split(':').collect();
                if parts.len() >= 2 && !parts[0].starts_with('/') {
                    volume_defs.push(parts[0].to_string());
                }
            }
        }

        let (networks, network_names) = Self::extract_networks(&network_settings, &host_config);

        let dns = if host_config.dns.as_ref().is_none_or(|d| d.is_empty()) {
            None
        } else {
            host_config.dns.clone()
        };

        let dns_search = if host_config
            .dns_search
            .as_ref()
            .is_none_or(|d| d.is_empty())
        {
            None
        } else {
            host_config.dns_search.clone()
        };

        let extra_hosts = if host_config
            .extra_hosts
            .as_ref()
            .is_none_or(|h| h.is_empty())
        {
            None
        } else {
            host_config.extra_hosts.clone()
        };

        let network_mode = host_config.network_mode.clone();

        let cap_add = if host_config.cap_add.as_ref().is_none_or(|c| c.is_empty()) {
            None
        } else {
            host_config.cap_add.clone()
        };

        let cap_drop = if host_config.cap_drop.as_ref().is_none_or(|c| c.is_empty()) {
            None
        } else {
            host_config.cap_drop.clone()
        };

        let security_opt = if host_config
            .security_opt
            .as_ref()
            .is_none_or(|s| s.is_empty())
        {
            None
        } else {
            host_config.security_opt.clone()
        };

        let restart = host_config.restart_policy.clone().and_then(|policy| {
            policy.name.map(|name| match name {
                RestartPolicyNameEnum::ALWAYS => "always".to_string(),
                RestartPolicyNameEnum::UNLESS_STOPPED => "unless-stopped".to_string(),
                RestartPolicyNameEnum::ON_FAILURE => {
                    if let Some(max_retry) = policy.maximum_retry_count {
                        if max_retry > 0 {
                            format!("on-failure:{}", max_retry)
                        } else {
                            "on-failure".to_string()
                        }
                    } else {
                        "on-failure".to_string()
                    }
                }
                RestartPolicyNameEnum::NO | RestartPolicyNameEnum::EMPTY => "no".to_string(),
            })
        });

        let deploy = Self::extract_deploy_config(&host_config);

        let healthcheck = config.healthcheck.as_ref().map(|hc| HealthCheck {
            test: hc.test.clone().unwrap_or_default(),
            interval: hc.interval.map(normalize_duration_from_ns),
            timeout: hc.timeout.map(normalize_duration_from_ns),
            retries: hc.retries,
            start_period: hc.start_period.map(normalize_duration_from_ns),
        });

        let labels = config.labels.and_then(filter_system_labels);

        let logging = host_config.log_config.as_ref().map(|log_config| Logging {
            driver: log_config.typ.clone().unwrap_or_default(),
            options: log_config.config.clone(),
        });

        let devices = host_config.devices.as_ref().and_then(|devs| {
            let device_list: Vec<String> = devs
                .iter()
                .map(|d| {
                    format!(
                        "{}:{}:{}",
                        d.path_on_host.clone().unwrap_or_default(),
                        d.path_in_container.clone().unwrap_or_default(),
                        d.cgroup_permissions.clone().unwrap_or_default()
                    )
                })
                .collect();
            if device_list.is_empty() {
                None
            } else {
                Some(device_list)
            }
        });

        let ulimits = host_config.ulimits.as_ref().and_then(|ulimit_list| {
            let ulimit_map: HashMap<String, UlimitConfig> = ulimit_list
                .iter()
                .filter_map(|ulimit| {
                    ulimit.name.as_ref().map(|name| {
                        (
                            name.clone(),
                            UlimitConfig {
                                soft: ulimit.soft.unwrap_or_default(),
                                hard: ulimit.hard.unwrap_or_default(),
                            },
                        )
                    })
                })
                .collect();
            if ulimit_map.is_empty() {
                None
            } else {
                Some(ulimit_map)
            }
        });

        let sysctls = host_config.sysctls.clone();

        // Extract new container attributes
        let init = host_config.init;
        let privileged = host_config.privileged;
        let tty = config.tty;
        let stdin_open = config.open_stdin;

        let user = config.user.filter(|u| !u.is_empty());
        let working_dir = config.working_dir.filter(|w| !w.is_empty());
        let entrypoint = config.entrypoint.map(|e| e.into_iter().collect());
        let command = config.cmd.map(|c| c.into_iter().collect());

        let service = Service {
            image,
            container_name: container_name.clone(),
            hostname,
            environment,
            ports,
            volumes,
            networks,
            network_mode,
            dns,
            dns_search,
            extra_hosts,
            restart,
            cap_add,
            cap_drop,
            security_opt,
            deploy,
            healthcheck,
            labels,
            logging,
            devices,
            user,
            working_dir,
            entrypoint,
            command,
            ulimits,
            sysctls,
            init,
            privileged,
            tty,
            stdin_open,
            depends_on: None, // Docker doesn't provide dependency info
        };

        let service_name = container_name.unwrap_or_else(|| "service".to_string());
        Ok((service_name, service, network_names, volume_defs))
    }

    fn extract_networks(
        network_settings: &bollard::models::NetworkSettings,
        host_config: &bollard::models::HostConfig,
    ) -> (Option<ServiceNetworks>, Vec<String>) {
        if let Some(network_mode) = &host_config.network_mode {
            if network_mode != "default" && network_mode != "bridge" {
                return (None, vec![]);
            }
        }

        if let Some(networks_map) = &network_settings.networks {
            let mut network_configs = HashMap::new();
            let mut network_names = Vec::new();

            for (net_name, net_info) in networks_map {
                network_names.push(net_name.clone());

                if let Some(ip) = &net_info.ip_address {
                    if !ip.is_empty() {
                        let mut config = NetworkConfig {
                            ipv4_address: Some(ip.clone()),
                            ipv6_address: None,
                        };

                        if let Some(ipv6) = &net_info.global_ipv6_address {
                            if !ipv6.is_empty() {
                                config.ipv6_address = Some(ipv6.clone());
                            }
                        }

                        network_configs.insert(net_name.clone(), config);
                    }
                }
            }

            let networks = if network_configs.is_empty() {
                if network_names.len() == 1 && network_names[0] == "bridge" {
                    None
                } else {
                    Some(ServiceNetworks::Simple(network_names.clone()))
                }
            } else {
                Some(ServiceNetworks::Advanced(network_configs))
            };

            (networks, network_names)
        } else {
            (None, vec![])
        }
    }

    fn extract_deploy_config(host_config: &bollard::models::HostConfig) -> Option<Deploy> {
        let mut has_resources = false;
        let mut limits = HashMap::new();

        if let Some(memory) = host_config.memory {
            if memory > 0 {
                let memory_mb = memory / (1024 * 1024);
                limits.insert("memory".to_string(), format!("{}M", memory_mb));
                has_resources = true;
            }
        }

        if let (Some(cpu_quota), Some(cpu_period)) = (host_config.cpu_quota, host_config.cpu_period)
        {
            if cpu_quota > 0 && cpu_period > 0 {
                let cpus = cpu_quota as f64 / cpu_period as f64;
                limits.insert("cpus".to_string(), format!("{:.2}", cpus));
                has_resources = true;
            }
        }

        let resources = if has_resources {
            Some(Resources {
                limits: Some(ResourceLimits {
                    memory: limits.get("memory").cloned(),
                    cpus: limits.get("cpus").cloned(),
                }),
            })
        } else {
            None
        };

        let placement = host_config
            .cpuset_cpus
            .as_ref()
            .filter(|cpus| !cpus.is_empty())
            .map(|cpus| Placement {
                constraints: Some(vec![format!("node.labels.cpus == {}", cpus)]),
            });

        if resources.is_some() || placement.is_some() {
            Some(Deploy {
                resources,
                placement,
            })
        } else {
            None
        }
    }
}
