/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

use crate::{
    filter_system_labels, normalize_duration, sanitize_service_name,
    security::{filter_sensitive_env_vars, validate_container_id, validate_image_id},
    AutoComposeError, Deploy, HealthCheck, Logging, NetworkConfig, ResourceLimits, Resources,
    Result, Service, ServiceNetworks, UlimitConfig,
};
use futures::stream::{FuturesUnordered, StreamExt};
use serde_json::Value;
use serde_yaml;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use tokio::process::Command;

pub struct PodmanProcessor;

impl Default for PodmanProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl PodmanProcessor {
    pub fn new() -> Self {
        Self
    }

    pub async fn list_containers(&self) -> Result<Vec<String>> {
        let output = Command::new("podman")
            .args(["ps", "-a", "--format", "{{.ID}}"])
            .output()
            .await?;

        if !output.status.success() {
            return Err(AutoComposeError::PodmanCommand(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let container_ids = String::from_utf8(output.stdout)?
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(container_ids)
    }

    pub async fn process_containers_parallel(
        &self,
        container_ids: Vec<String>,
    ) -> Result<(
        HashMap<String, Service>,
        HashMap<String, serde_yaml::Value>,
        HashMap<String, serde_yaml::Value>,
    )> {
        let mut inspect_tasks = FuturesUnordered::new();

        for id in container_ids {
            inspect_tasks.push(async move { Self::inspect_container(id).await });
        }

        let mut services = HashMap::new();
        let mut global_networks = HashMap::new();
        let volumes = HashMap::new();

        while let Some(result) = inspect_tasks.next().await {
            match result {
                Ok(inspect_data) => {
                    if let Some(container) = inspect_data.get(0) {
                        if let Ok((service_name, service, _networks, network_configs)) =
                            Self::translate_container(container).await
                        {
                            services.insert(service_name, service);

                            for (net_name, net_config) in network_configs {
                                global_networks.insert(net_name, net_config);
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Error during container inspection: {:?}", e),
            }
        }

        Ok((services, global_networks, volumes))
    }

    async fn inspect_container(id: String) -> Result<Value> {
        // Validate container ID to prevent command injection
        let safe_id = validate_container_id(&id)?;

        let output = Command::new("podman")
            .args(["inspect", safe_id])
            .output()
            .await?;

        if !output.status.success() {
            return Err(AutoComposeError::PodmanCommand(format!(
                "Failed to inspect container {}: {}",
                safe_id,
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let inspect_data: Value = serde_json::from_slice(&output.stdout)?;
        Ok(inspect_data)
    }

    async fn get_image_repo(image_id: &str) -> Option<String> {
        // Validate image ID to prevent command injection
        let safe_image_id = validate_image_id(image_id).ok()?;

        let output = Command::new("podman")
            .args(["image", "inspect", safe_image_id])
            .output()
            .await
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let image_info: Value = serde_json::from_slice(&output.stdout).ok()?;
        let repo_tags = image_info.get(0)?.get("RepoTags")?;

        if let Some(arr) = repo_tags.as_array() {
            if let Some(first) = arr.first()?.as_str() {
                return Some(first.to_string());
            }
        }
        None
    }

    fn compute_subnet(gateway: &str, prefix_len: u64) -> Option<String> {
        if prefix_len > 32 {
            return None;
        }
        let gw: Ipv4Addr = gateway.parse().ok()?;
        let gw_u32: u32 = gw.into();
        let mask: u32 = if prefix_len == 0 {
            0
        } else {
            !0u32 << (32 - prefix_len as u32)
        };
        let network_u32 = gw_u32 & mask;
        let network = Ipv4Addr::from(network_u32);
        Some(format!("{}/{}", network, prefix_len))
    }

    async fn translate_container(
        container: &Value,
    ) -> Result<(
        String,
        Service,
        Vec<String>,
        HashMap<String, serde_yaml::Value>,
    )> {
        let name = container["Name"]
            .as_str()
            .unwrap_or("unnamed")
            .trim_start_matches('/');
        let service_name = sanitize_service_name(name);

        let mut image = container["Image"].as_str().unwrap_or("unknown").to_string();

        if image.len() == 64 && image.chars().all(|c| c.is_ascii_hexdigit()) {
            if let Some(repo) = Self::get_image_repo(&image).await {
                image = repo;
            }
        }

        let hostname = container["Config"]["Hostname"]
            .as_str()
            .map(|h| h.to_string())
            .filter(|h| !h.is_empty());

        let environment = container["Config"]["Env"].as_array().and_then(|env| {
            let env_map: HashMap<String, String> = env
                .iter()
                .filter_map(|v| v.as_str())
                .filter_map(|s| {
                    let parts: Vec<&str> = s.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        Some((parts[0].to_string(), parts[1].to_string()))
                    } else {
                        None
                    }
                })
                .collect();
            // Filter out sensitive environment variables
            let filtered_env = filter_sensitive_env_vars(env_map);
            if filtered_env.is_empty() {
                None
            } else {
                Some(filtered_env)
            }
        });

        let ports = Self::extract_ports(container);

        let volumes = Self::extract_volumes(container);

        let (networks, network_configs) = Self::extract_networks(container);

        let network_mode = container["HostConfig"]["NetworkMode"]
            .as_str()
            .filter(|mode| *mode != "default")
            .map(|mode| mode.to_string());

        let dns = container["HostConfig"]["Dns"].as_array().and_then(|dns| {
            let dns_servers: Vec<String> = dns
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect();
            if dns_servers.is_empty() {
                None
            } else {
                Some(dns_servers)
            }
        });

        let dns_search = container["HostConfig"]["DnsSearch"]
            .as_array()
            .and_then(|dns_search| {
                let searches: Vec<String> = dns_search
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect();
                if searches.is_empty() {
                    None
                } else {
                    Some(searches)
                }
            });

        let extra_hosts = container["HostConfig"]["ExtraHosts"]
            .as_array()
            .and_then(|hosts| {
                let hosts_vec: Vec<String> = hosts
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect();
                if hosts_vec.is_empty() {
                    None
                } else {
                    Some(hosts_vec)
                }
            });

        let restart = Self::extract_restart_policy(container);

        let cap_add = container["HostConfig"]["CapAdd"]
            .as_array()
            .and_then(|caps| {
                let capabilities: Vec<String> = caps
                    .iter()
                    .filter_map(|c| c.as_str())
                    .map(|s| s.to_string())
                    .collect();
                if capabilities.is_empty() {
                    None
                } else {
                    Some(capabilities)
                }
            });

        let cap_drop = container["HostConfig"]["CapDrop"]
            .as_array()
            .and_then(|caps| {
                let capabilities: Vec<String> = caps
                    .iter()
                    .filter_map(|c| c.as_str())
                    .map(|s| s.to_string())
                    .collect();
                if capabilities.is_empty() {
                    None
                } else {
                    Some(capabilities)
                }
            });

        let security_opt = container["HostConfig"]["SecurityOpt"]
            .as_array()
            .and_then(|opts| {
                let options: Vec<String> = opts
                    .iter()
                    .filter_map(|s| s.as_str())
                    .map(|s| s.to_string())
                    .collect();
                if options.is_empty() {
                    None
                } else {
                    Some(options)
                }
            });

        let deploy = Self::extract_deploy_config(container);

        let healthcheck = Self::extract_healthcheck(container);

        let labels = container["Config"]["Labels"]
            .as_object()
            .and_then(|labels| {
                let label_map: HashMap<String, String> = labels
                    .iter()
                    .filter_map(|(key, value)| value.as_str().map(|v| (key.clone(), v.to_string())))
                    .collect();
                filter_system_labels(label_map)
            });

        let logging = Self::extract_logging(container);

        let devices = Self::extract_devices(container);

        let ulimits = Self::extract_ulimits(container);

        let sysctls = container["HostConfig"]["Sysctls"]
            .as_object()
            .and_then(|sysctls| {
                let sysctls_map: HashMap<String, String> = sysctls
                    .iter()
                    .filter_map(|(key, value)| value.as_str().map(|v| (key.clone(), v.to_string())))
                    .collect();
                if sysctls_map.is_empty() {
                    None
                } else {
                    Some(sysctls_map)
                }
            });

        // Extract new container attributes
        let init = container["HostConfig"]["Init"].as_bool();
        let privileged = container["HostConfig"]["Privileged"].as_bool();
        let tty = container["Config"]["Tty"].as_bool();
        let stdin_open = container["Config"]["OpenStdin"].as_bool();

        let user = container["Config"]["User"]
            .as_str()
            .filter(|u| !u.is_empty())
            .map(|u| u.to_string());

        let working_dir = container["Config"]["WorkingDir"]
            .as_str()
            .filter(|w| !w.is_empty())
            .map(|w| w.to_string());

        let entrypoint = container["Config"]["Entrypoint"].as_array().and_then(|ep| {
            let ep_vec: Vec<String> = ep
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect();
            if ep_vec.is_empty() {
                None
            } else {
                Some(ep_vec)
            }
        });

        let command = container["Config"]["Cmd"].as_array().and_then(|cmd| {
            let cmd_vec: Vec<String> = cmd
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect();
            if cmd_vec.is_empty() {
                None
            } else {
                Some(cmd_vec)
            }
        });

        let service = Service {
            image,
            container_name: Some(service_name.clone()),
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
            depends_on: None, // Podman doesn't provide dependency info
        };

        Ok((service_name, service, vec![], network_configs))
    }

    fn extract_ports(container: &Value) -> Option<Vec<String>> {
        container["NetworkSettings"]["Ports"]
            .as_object()
            .and_then(|ports| {
                let port_mappings: Vec<String> = ports
                    .iter()
                    .filter_map(|(container_port, host_bindings)| {
                        host_bindings.as_array().map(|bindings| {
                            bindings
                                .iter()
                                .filter_map(|binding| {
                                    let host_ip = binding["HostIp"].as_str().unwrap_or("0.0.0.0");
                                    let host_port = binding["HostPort"].as_str()?;

                                    Some(if host_ip == "0.0.0.0" {
                                        format!("{}:{}", host_port, container_port)
                                    } else {
                                        format!("{}:{}:{}", host_ip, host_port, container_port)
                                    })
                                })
                                .collect::<Vec<String>>()
                        })
                    })
                    .flatten()
                    .collect();
                if port_mappings.is_empty() {
                    None
                } else {
                    Some(port_mappings)
                }
            })
    }

    fn extract_volumes(container: &Value) -> Option<Vec<String>> {
        container["Mounts"].as_array().and_then(|mounts| {
            let volume_mappings: Vec<String> = mounts
                .iter()
                .filter_map(|mount| {
                    let src = mount["Source"].as_str()?;
                    let dst = mount["Destination"].as_str()?;
                    let mount_type = mount["Type"].as_str().unwrap_or("bind");

                    Some(match mount_type {
                        "tmpfs" => format!("tmpfs:{}", dst),
                        _ => format!("{}:{}", src, dst),
                    })
                })
                .collect();
            if volume_mappings.is_empty() {
                None
            } else {
                Some(volume_mappings)
            }
        })
    }

    fn extract_networks(
        container: &Value,
    ) -> (Option<ServiceNetworks>, HashMap<String, serde_yaml::Value>) {
        let mut network_configs = HashMap::new();
        let mut service_networks = HashMap::new();

        if let Some(networks_obj) = container["NetworkSettings"]["Networks"].as_object() {
            for (net_name, net_info) in networks_obj {
                if let (Some(gateway), Some(prefix_len)) = (
                    net_info["Gateway"].as_str(),
                    net_info["IPPrefixLen"].as_u64(),
                ) {
                    if let Some(subnet) = Self::compute_subnet(gateway, prefix_len) {
                        let mut network_config = serde_yaml::Mapping::new();
                        let mut ipam_entry = serde_yaml::Mapping::new();

                        ipam_entry.insert(
                            serde_yaml::Value::String("subnet".to_string()),
                            serde_yaml::Value::String(subnet),
                        );
                        ipam_entry.insert(
                            serde_yaml::Value::String("gateway".to_string()),
                            serde_yaml::Value::String(gateway.to_string()),
                        );

                        let ipam_config =
                            serde_yaml::Value::Sequence(vec![serde_yaml::Value::Mapping(
                                ipam_entry,
                            )]);
                        let mut ipam_map = serde_yaml::Mapping::new();
                        ipam_map
                            .insert(serde_yaml::Value::String("config".to_string()), ipam_config);
                        network_config.insert(
                            serde_yaml::Value::String("ipam".to_string()),
                            serde_yaml::Value::Mapping(ipam_map),
                        );

                        network_configs
                            .insert(net_name.clone(), serde_yaml::Value::Mapping(network_config));
                    }
                }

                if let Some(ip) = net_info["IPAddress"].as_str() {
                    if !ip.is_empty() {
                        service_networks.insert(
                            net_name.clone(),
                            NetworkConfig {
                                ipv4_address: Some(ip.to_string()),
                                ipv6_address: None,
                            },
                        );
                    }
                }
            }
        }

        let networks = if service_networks.is_empty() {
            None
        } else {
            Some(ServiceNetworks::Advanced(service_networks))
        };

        (networks, network_configs)
    }

    fn extract_restart_policy(container: &Value) -> Option<String> {
        container["HostConfig"]["RestartPolicy"]["Name"]
            .as_str()
            .map(|policy| match policy {
                "always" => "always".to_string(),
                "unless-stopped" => "unless-stopped".to_string(),
                "on-failure" => {
                    if let Some(max_retry) =
                        container["HostConfig"]["RestartPolicy"]["MaximumRetryCount"].as_u64()
                    {
                        if max_retry > 0 {
                            format!("on-failure:{}", max_retry)
                        } else {
                            "on-failure".to_string()
                        }
                    } else {
                        "on-failure".to_string()
                    }
                }
                _ => "no".to_string(),
            })
    }

    fn extract_deploy_config(container: &Value) -> Option<Deploy> {
        let mut has_resources = false;
        let mut memory = None;
        let mut cpus = None;

        if let Some(mem) = container["HostConfig"]["Memory"].as_u64() {
            if mem > 0 {
                let memory_mb = mem / (1024 * 1024);
                memory = Some(format!("{}M", memory_mb));
                has_resources = true;
            }
        }

        if let (Some(cpu_quota), Some(cpu_period)) = (
            container["HostConfig"]["CpuQuota"].as_i64(),
            container["HostConfig"]["CpuPeriod"].as_i64(),
        ) {
            if cpu_quota > 0 && cpu_period > 0 {
                let cpu_value = cpu_quota as f64 / cpu_period as f64;
                cpus = Some(format!("{:.2}", cpu_value));
                has_resources = true;
            }
        }

        if has_resources {
            Some(Deploy {
                resources: Some(Resources {
                    limits: Some(ResourceLimits { memory, cpus }),
                }),
                placement: None,
            })
        } else {
            None
        }
    }

    fn extract_healthcheck(container: &Value) -> Option<HealthCheck> {
        container["Config"]["Healthcheck"]
            .as_object()
            .and_then(|hc| {
                let test = hc
                    .get("Test")
                    .and_then(|v| v.as_array())
                    .map(|test_array| {
                        test_array
                            .iter()
                            .filter_map(|v| v.as_str())
                            .map(|s| s.to_string())
                            .collect()
                    })?;

                Some(HealthCheck {
                    test,
                    interval: hc
                        .get("Interval")
                        .and_then(|v| v.as_str())
                        .map(normalize_duration),
                    timeout: hc
                        .get("Timeout")
                        .and_then(|v| v.as_str())
                        .map(normalize_duration),
                    retries: hc.get("Retries").and_then(|v| v.as_i64()),
                    start_period: hc
                        .get("StartPeriod")
                        .and_then(|v| v.as_str())
                        .map(normalize_duration),
                })
            })
    }

    fn extract_logging(container: &Value) -> Option<Logging> {
        container["HostConfig"]["LogConfig"]
            .as_object()
            .and_then(|log_config| {
                let driver = log_config.get("Type")?.as_str()?.to_string();
                let options = log_config
                    .get("Config")
                    .and_then(|config| config.as_object())
                    .and_then(|config_obj| {
                        let opts: HashMap<String, String> = config_obj
                            .iter()
                            .filter_map(|(key, value)| {
                                value.as_str().map(|v| (key.clone(), v.to_string()))
                            })
                            .collect();
                        if opts.is_empty() {
                            None
                        } else {
                            Some(opts)
                        }
                    });

                Some(Logging { driver, options })
            })
    }

    fn extract_devices(container: &Value) -> Option<Vec<String>> {
        container["HostConfig"]["Devices"]
            .as_array()
            .and_then(|devices| {
                let device_mappings: Vec<String> = devices
                    .iter()
                    .filter_map(|d| {
                        let host_path = d["PathOnHost"].as_str()?;
                        let container_path = d["PathInContainer"].as_str()?;
                        Some(format!("{}:{}", host_path, container_path))
                    })
                    .collect();
                if device_mappings.is_empty() {
                    None
                } else {
                    Some(device_mappings)
                }
            })
    }

    fn extract_ulimits(container: &Value) -> Option<HashMap<String, UlimitConfig>> {
        container["HostConfig"]["Ulimits"]
            .as_array()
            .and_then(|ulimits| {
                let ulimit_map: HashMap<String, UlimitConfig> = ulimits
                    .iter()
                    .filter_map(|ulimit| {
                        let name = ulimit["Name"].as_str()?;
                        let soft = ulimit["Soft"].as_i64()?;
                        let hard = ulimit["Hard"].as_i64()?;
                        Some((name.to_string(), UlimitConfig { soft, hard }))
                    })
                    .collect();
                if ulimit_map.is_empty() {
                    None
                } else {
                    Some(ulimit_map)
                }
            })
    }
}
