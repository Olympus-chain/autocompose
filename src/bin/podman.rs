use clap::Parser;
use futures::stream::{FuturesUnordered, StreamExt};
use serde::{Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use tokio::process::Command;

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum ComposeValue {
    String(String),
    Vec(Vec<String>),
    Map(HashMap<String, ComposeValue>),
}

#[derive(Serialize)]
struct ComposeFile {
    version: String,
    services: HashMap<String, HashMap<String, ComposeValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<HashMap<String, serde_yaml::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    networks: Option<HashMap<String, serde_yaml::Value>>,
}

#[derive(Parser)]
#[command(
    name = "Podman to Docker Compose Exporter",
    about = "Exports Podman containers to a Docker Compose file with in-depth translation, including network configuration and image resolution."
)]
struct Args {
    #[arg(short, long, default_value = "docker-compose.yml")]
    output: String,
    #[arg(short, long, default_value = "3.9")]
    version: String,
}


fn compute_subnet(gateway: &str, prefix_len: u64) -> Option<String> {
    if prefix_len > 32 {
        return None;
    }
    let gw: Ipv4Addr = gateway.parse().ok()?;
    let gw_u32: u32 = gw.into();
    let mask: u32 = if prefix_len == 0 { 0 } else { !0u32 << (32 - prefix_len as u32) };
    let network_u32 = gw_u32 & mask;
    let network = Ipv4Addr::from(network_u32);
    Some(format!("{}/{}", network, prefix_len))
}


async fn inspect_container(id: String) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let output = Command::new("podman")
        .args(["inspect", &id])
        .output()
        .await?;
    let inspect_data: Value = serde_json::from_slice(&output.stdout)?;
    Ok(inspect_data)
}


async fn get_image_repo(image_id: &str) -> Option<String> {
    let output = Command::new("podman")
        .args(["image", "inspect", image_id])
        .output()
        .await
        .ok()?;
    let image_info: Value = serde_json::from_slice(&output.stdout).ok()?;

    let repo_tags = image_info.get(0)?.get("RepoTags")?;
    if let Some(arr) = repo_tags.as_array() {
        if let Some(first) = arr.first()?.as_str() {
            return Some(first.to_string());
        }
    }
    None
}


fn translate_container(container: &Value) -> HashMap<String, ComposeValue> {
    let mut service: HashMap<String, ComposeValue> = HashMap::new();


    let name = container["Name"]
        .as_str()
        .unwrap_or("unnamed")
        .trim_start_matches('/')
        .to_string();
    service.insert("container_name".to_string(), ComposeValue::String(name.clone()));


    if let Some(image) = container["Image"].as_str() {
        service.insert("image".to_string(), ComposeValue::String(image.to_string()));
    }

    // Entrypoint
    if let Some(cmd) = container["Config"]["Cmd"].as_array() {
        let cmd_str = cmd.iter().filter_map(|v| v.as_str()).collect::<Vec<&str>>().join(" ");
        if !cmd_str.is_empty() {
            service.insert("command".to_string(), ComposeValue::String(cmd_str));
        }
    }
    if let Some(entrypoint) = container["Config"]["Entrypoint"].as_array() {
        let entrypoint_str = entrypoint.iter().filter_map(|v| v.as_str()).collect::<Vec<&str>>().join(" ");
        if !entrypoint_str.is_empty() {
            service.insert("entrypoint".to_string(), ComposeValue::String(entrypoint_str));
        }
    }

    // Environnement
    if let Some(env) = container["Config"]["Env"].as_array() {
        let env_vars = env.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect::<Vec<String>>();
        if !env_vars.is_empty() {
            service.insert("environment".to_string(), ComposeValue::Vec(env_vars));
        }
    }

    // Ports
    if let Some(ports) = container["NetworkSettings"]["Ports"].as_object() {
        let mut port_mappings = Vec::new();
        for (container_port, host_bindings) in ports {
            if let Some(bindings) = host_bindings.as_array() {
                for binding in bindings {
                    if let (Some(host_ip), Some(host_port)) = (binding["HostIp"].as_str(), binding["HostPort"].as_str()) {
                        let mapping = if host_ip == "0.0.0.0" {
                            format!("{}:{}", host_port, container_port)
                        } else {
                            format!("{}:{}:{}", host_ip, host_port, container_port)
                        };
                        port_mappings.push(mapping);
                    }
                }
            }
        }
        if !port_mappings.is_empty() {
            service.insert("ports".to_string(), ComposeValue::Vec(port_mappings));
        }
    }

    if let Some(nw_mode) = container["HostConfig"]["NetworkMode"].as_str() {
        if nw_mode != "default" {
            service.insert("network_mode".to_string(), ComposeValue::String(nw_mode.to_string()));
        } else if let Some(networks_obj) = container["NetworkSettings"]["Networks"].as_object() {
            let mut service_networks = HashMap::new();
            for (net_name, net_info) in networks_obj {
                if let Some(ip) = net_info["IPAddress"].as_str() {
                    let mut net_config = HashMap::new();
                    net_config.insert("ipv4_address".to_string(), ComposeValue::String(ip.to_string()));
                    service_networks.insert(net_name.to_string(), ComposeValue::Map(net_config));
                } else {
                    service_networks.insert(net_name.to_string(), ComposeValue::String(net_name.to_string()));
                }
            }
            if !service_networks.is_empty() {
                service.insert("networks".to_string(), ComposeValue::Map(service_networks));
            }
        }
    } else if let Some(networks_obj) = container["NetworkSettings"]["Networks"].as_object() {
        let mut service_networks = HashMap::new();
        for (net_name, net_info) in networks_obj {
            if let Some(ip) = net_info["IPAddress"].as_str() {
                let mut net_config = HashMap::new();
                net_config.insert("ipv4_address".to_string(), ComposeValue::String(ip.to_string()));
                service_networks.insert(net_name.to_string(), ComposeValue::Map(net_config));
            } else {
                service_networks.insert(net_name.to_string(), ComposeValue::String(net_name.to_string()));
            }
        }
        if !service_networks.is_empty() {
            service.insert("networks".to_string(), ComposeValue::Map(service_networks));
        }
    }

    // Volumes
    if let Some(mounts) = container["Mounts"].as_array() {
        let mut volume_mappings = Vec::new();
        for mount in mounts {
            if let (Some(src), Some(dst)) = (mount["Source"].as_str(), mount["Destination"].as_str()) {
                let mount_type = mount["Type"].as_str().unwrap_or("bind");
                let mapping = match mount_type {
                    "volume" => format!("{}:{}", src, dst),
                    "bind" => format!("{}:{}", src, dst),
                    "tmpfs" => format!("{}:{}", dst, "tmpfs"),
                    _ => format!("{}:{}", src, dst),
                };
                volume_mappings.push(mapping);
            }
        }
        if !volume_mappings.is_empty() {
            service.insert("volumes".to_string(), ComposeValue::Vec(volume_mappings));
        }
    }

    if let Some(restart_policy) = container["HostConfig"]["RestartPolicy"]["Name"].as_str() {
        let restart = match restart_policy {
            "always" => "always".to_string(),
            "unless-stopped" => "unless-stopped".to_string(),
            "on-failure" => {
                if let Some(max_retry) = container["HostConfig"]["RestartPolicy"]["MaximumRetryCount"].as_u64() {
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
        };
        service.insert("restart".to_string(), ComposeValue::String(restart));
    }


    if let Some(labels) = container["Config"]["Labels"].as_object() {
        let mut label_map = HashMap::new();
        for (key, value) in labels {
            if key.starts_with("io.buildah") {
                continue;
            }
            if let Some(v) = value.as_str() {
                label_map.insert(key.clone(), ComposeValue::String(v.to_string()));
            }
        }
        if !label_map.is_empty() {
            service.insert("labels".to_string(), ComposeValue::Map(label_map));
        }
    }

    // Logging
    if let Some(log_config) = container["HostConfig"]["LogConfig"].as_object() {
        if let Some(driver) = log_config["Type"].as_str() {
            let mut logging = HashMap::new();
            logging.insert("driver".to_string(), ComposeValue::String(driver.to_string()));
            if let Some(opts) = log_config["Config"].as_object() {
                let mut options = HashMap::new();
                for (key, value) in opts {
                    if let Some(v) = value.as_str() {
                        options.insert(key.clone(), ComposeValue::String(v.to_string()));
                    }
                }
                if !options.is_empty() {
                    logging.insert("options".to_string(), ComposeValue::Map(options));
                }
            }
            service.insert("logging".to_string(), ComposeValue::Map(logging));
        }
    }

    if let Some(user) = container["Config"]["User"].as_str() {
        if !user.is_empty() {
            service.insert("user".to_string(), ComposeValue::String(user.to_string()));
        }
    }

    if let Some(workdir) = container["Config"]["WorkingDir"].as_str() {
        if !workdir.is_empty() {
            service.insert("working_dir".to_string(), ComposeValue::String(workdir.to_string()));
        }
    }

    if let Some(hostname) = container["Config"]["Hostname"].as_str() {
        service.insert("hostname".to_string(), ComposeValue::String(hostname.to_string()));
    }


    if let Some(devices) = container["HostConfig"]["Devices"].as_array() {
        let device_mappings = devices
            .iter()
            .filter_map(|d| {
                if let (Some(path_on_host), Some(path_in_container)) =
                    (d["PathOnHost"].as_str(), d["PathInContainer"].as_str())
                {
                    Some(format!("{}:{}", path_on_host, path_in_container))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();
        if !device_mappings.is_empty() {
            service.insert("devices".to_string(), ComposeValue::Vec(device_mappings));
        }
    }

    //  (cap_add)
    if let Some(cap_add) = container["HostConfig"]["CapAdd"].as_array() {
        let capabilities = cap_add
            .iter()
            .filter_map(|c| c.as_str())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if !capabilities.is_empty() {
            service.insert("cap_add".to_string(), ComposeValue::Vec(capabilities));
        }
    }

    // (cap_drop)
    if let Some(cap_drop) = container["HostConfig"]["CapDrop"].as_array() {
        let capabilities_drop = cap_drop
            .iter()
            .filter_map(|c| c.as_str())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if !capabilities_drop.is_empty() {
            service.insert("cap_drop".to_string(), ComposeValue::Vec(capabilities_drop));
        }
    }

    // (security_opt)
    if let Some(security_opt) = container["HostConfig"]["SecurityOpt"].as_array() {
        let security_options = security_opt
            .iter()
            .filter_map(|s| s.as_str())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if !security_options.is_empty() {
            service.insert("security_opt".to_string(), ComposeValue::Vec(security_options));
        }
    }

    // Ulimits
    if let Some(ulimits) = container["HostConfig"]["Ulimits"].as_array() {
        let mut ulimit_map = HashMap::new();
        for ulimit in ulimits {
            if let (Some(name), Some(soft), Some(hard)) = (
                ulimit["Name"].as_str(),
                ulimit["Soft"].as_i64(),
                ulimit["Hard"].as_i64(),
            ) {
                let mut limit_detail = HashMap::new();
                limit_detail.insert("soft".to_string(), ComposeValue::String(soft.to_string()));
                limit_detail.insert("hard".to_string(), ComposeValue::String(hard.to_string()));
                ulimit_map.insert(name.to_string(), ComposeValue::Map(limit_detail));
            }
        }
        if !ulimit_map.is_empty() {
            service.insert("ulimits".to_string(), ComposeValue::Map(ulimit_map));
        }
    }

    // Sysctls
    if let Some(sysctls) = container["HostConfig"]["Sysctls"].as_object() {
        let mut sysctls_map = HashMap::new();
        for (key, value) in sysctls {
            if let Some(val) = value.as_str() {
                sysctls_map.insert(key.clone(), ComposeValue::String(val.to_string()));
            }
        }
        if !sysctls_map.is_empty() {
            service.insert("sysctls".to_string(), ComposeValue::Map(sysctls_map));
        }
    }

    if let Some(dns) = container["HostConfig"]["Dns"].as_array() {
        let dns_servers = dns
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if !dns_servers.is_empty() {
            service.insert("dns".to_string(), ComposeValue::Vec(dns_servers));
        }
    }
    if let Some(dns_search) = container["HostConfig"]["DnsSearch"].as_array() {
        let dns_searches = dns_search
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if !dns_searches.is_empty() {
            service.insert("dns_search".to_string(), ComposeValue::Vec(dns_searches));
        }
    }
    if let Some(extra_hosts) = container["HostConfig"]["ExtraHosts"].as_array() {
        let extra_hosts_vec = extra_hosts
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if !extra_hosts_vec.is_empty() {
            service.insert("extra_hosts".to_string(), ComposeValue::Vec(extra_hosts_vec));
        }
    }

    let mut deploy_map = HashMap::new();
    let mut resources_map = HashMap::new();
    let mut limits_map = HashMap::new();

    if let Some(memory) = container["HostConfig"]["Memory"].as_u64() {
        if memory > 0 {
            let memory_mb = memory / (1024 * 1024);
            limits_map.insert("memory".to_string(), ComposeValue::String(format!("{}M", memory_mb)));
        }
    }
    if let (Some(cpu_quota), Some(cpu_period)) = (
        container["HostConfig"]["CpuQuota"].as_i64(),
        container["HostConfig"]["CpuPeriod"].as_i64(),
    ) {
        if cpu_quota > 0 && cpu_period > 0 {
            let cpus = (cpu_quota as f64) / (cpu_period as f64);
            limits_map.insert("cpus".to_string(), ComposeValue::String(format!("{:.2}", cpus)));
        }
    }
    if !limits_map.is_empty() {
        resources_map.insert("limits".to_string(), ComposeValue::Map(limits_map));
    }
    if !resources_map.is_empty() {
        deploy_map.insert("resources".to_string(), ComposeValue::Map(resources_map));
    }
    if !deploy_map.is_empty() {
        service.insert("deploy".to_string(), ComposeValue::Map(deploy_map));
    }

    // Healthcheck
    if let Some(healthcheck) = container["Config"]["Healthcheck"].as_object() {
        let mut hc_map = HashMap::new();
        if let Some(test) = healthcheck.get("Test").and_then(|v| v.as_array()) {
            let tests = test.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect::<Vec<String>>();
            if !tests.is_empty() {
                hc_map.insert("test".to_string(), ComposeValue::Vec(tests));
            }
        }
        if let Some(interval) = healthcheck.get("Interval").and_then(|v| v.as_str()) {
            hc_map.insert("interval".to_string(), ComposeValue::String(interval.to_string()));
        }
        if let Some(timeout) = healthcheck.get("Timeout").and_then(|v| v.as_str()) {
            hc_map.insert("timeout".to_string(), ComposeValue::String(timeout.to_string()));
        }
        if let Some(retries) = healthcheck.get("Retries").and_then(|v| v.as_u64()) {
            hc_map.insert("retries".to_string(), ComposeValue::String(retries.to_string()));
        }
        if let Some(start_period) = healthcheck.get("StartPeriod").and_then(|v| v.as_str()) {
            hc_map.insert("start_period".to_string(), ComposeValue::String(start_period.to_string()));
        }
        if !hc_map.is_empty() {
            service.insert("healthcheck".to_string(), ComposeValue::Map(hc_map));
        }
    }

    service
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();

    let mut global_networks: HashMap<String, serde_yaml::Value> = HashMap::new();

    let ps_output = Command::new("podman")
        .args(["ps", "-a", "--format", "{{.ID}}"])
        .output()
        .await?;
    let container_ids = String::from_utf8(ps_output.stdout)?
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut inspect_tasks = FuturesUnordered::new();
    for id in container_ids {
        inspect_tasks.push(async move { inspect_container(id).await });
    }

    let mut services: HashMap<String, HashMap<String, ComposeValue>> = HashMap::new();

    while let Some(result) = inspect_tasks.next().await {
        match result {
            Ok(inspect_data) => {
                if let Some(container) = inspect_data.get(0) {
                    if let Some(networks_obj) = container["NetworkSettings"]["Networks"].as_object() {
                        for (net_name, net_info) in networks_obj {
                            if !global_networks.contains_key(net_name) {
                                let mut network_config = serde_yaml::Mapping::new();
                                if let Some(gateway) = net_info["Gateway"].as_str() {
                                    if let Some(prefix_len) = net_info["IPPrefixLen"].as_u64() {
                                        if let Some(subnet) = compute_subnet(gateway, prefix_len) {
                                            let mut ipam_entry = serde_yaml::Mapping::new();
                                            ipam_entry.insert(
                                                serde_yaml::Value::String("subnet".to_string()),
                                                serde_yaml::Value::String(subnet),
                                            );
                                            ipam_entry.insert(
                                                serde_yaml::Value::String("gateway".to_string()),
                                                serde_yaml::Value::String(gateway.to_string()),
                                            );
                                            let ipam_config = serde_yaml::Value::Sequence(vec![serde_yaml::Value::Mapping(ipam_entry)]);
                                            let mut ipam_map = serde_yaml::Mapping::new();
                                            ipam_map.insert(
                                                serde_yaml::Value::String("config".to_string()),
                                                ipam_config,
                                            );
                                            network_config.insert(
                                                serde_yaml::Value::String("ipam".to_string()),
                                                serde_yaml::Value::Mapping(ipam_map),
                                            );
                                        }
                                    }
                                }
                                global_networks.insert(net_name.clone(), serde_yaml::Value::Mapping(network_config));
                            }
                        }
                    }
                    let service = translate_container(container);
                    if let Some(ComposeValue::String(name)) = service.get("container_name") {
                        services.insert(name.clone(), service);
                    }
                }
            }
            Err(e) => eprintln!("Error during container inspection: {:?}", e),
        }
    }

    for service in services.values_mut() {
        if let Some(ComposeValue::String(image_val)) = service.get_mut("image") {
            if image_val.len() == 64 && image_val.chars().all(|c| c.is_ascii_hexdigit()) {
                if let Some(repo) = get_image_repo(image_val).await {
                    *image_val = repo;
                }
            }
        }
    }

    let compose = ComposeFile {
        version: args.version,
        services,
        volumes: None,
        networks: if global_networks.is_empty() { None } else { Some(global_networks) },
    };

    let yaml = serde_yaml::to_string(&compose)?;
    tokio::fs::write(&args.output, yaml).await?;
    println!("Docker Compose file generated: {}", args.output);

    Ok(())
}
