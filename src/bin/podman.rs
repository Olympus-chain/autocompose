use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::Command;

#[derive(Debug)]
enum ComposeValue {
    String(String),
    Vec(Vec<String>),
    Map(HashMap<String, ComposeValue>),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("podman")
        .args(&["ps", "-a", "--format", "{{.ID}}"])
        .output()?;

    let container_ids = String::from_utf8(output.stdout)?
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut services = HashMap::new();
    let volumes: HashMap<String, HashMap<String, String>> = HashMap::new();
    let networks: HashMap<String, HashMap<String, String>> = HashMap::new();

    for id in container_ids {
        let inspect_output = Command::new("podman").args(&["inspect", &id]).output()?;

        let inspect_data: Value = serde_json::from_slice(&inspect_output.stdout)?;

        if let Some(container) = inspect_data.get(0) {
            let name = container["Name"]
                .as_str()
                .unwrap_or("unnamed")
                .trim_start_matches('/');
            let mut service = HashMap::new();

            // Image
            if let Some(image) = container["Image"].as_str() {
                service.insert("image".to_string(), ComposeValue::String(image.to_string()));
            }

            // Container name
            service.insert(
                "container_name".to_string(),
                ComposeValue::String(name.to_string()),
            );

            // Command
            if let Some(cmd) = container["Config"]["Cmd"].as_array() {
                let cmd_str = cmd
                    .iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<&str>>()
                    .join(" ");
                service.insert("command".to_string(), ComposeValue::String(cmd_str));
            }

            // Entrypoint
            if let Some(entrypoint) = container["Config"]["Entrypoint"].as_array() {
                let entrypoint_str = entrypoint
                    .iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<&str>>()
                    .join(" ");
                service.insert(
                    "entrypoint".to_string(),
                    ComposeValue::String(entrypoint_str),
                );
            }

            // Environment
            if let Some(env) = container["Config"]["Env"].as_array() {
                let env_vars = env
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                service.insert("environment".to_string(), ComposeValue::Vec(env_vars));
            }

            // Ports
            if let Some(ports) = container["NetworkSettings"]["Ports"].as_object() {
                let mut port_mappings = Vec::new();
                for (container_port, host_bindings) in ports {
                    if let Some(bindings) = host_bindings.as_array() {
                        for binding in bindings {
                            if let (Some(host_ip), Some(host_port)) =
                                (binding["HostIp"].as_str(), binding["HostPort"].as_str())
                            {
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

            // Volumes
            if let Some(mounts) = container["Mounts"].as_array() {
                let volume_mappings = mounts
                    .iter()
                    .filter_map(|m| {
                        if let (Some(src), Some(dst), Some(mode)) = (
                            m["Source"].as_str(),
                            m["Destination"].as_str(),
                            m["Mode"].as_str(),
                        ) {
                            Some(format!("{}:{}:{}", src, dst, mode))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>();
                if !volume_mappings.is_empty() {
                    service.insert("volumes".to_string(), ComposeValue::Vec(volume_mappings));
                }
            }

            // Restart policy
            if let Some(restart_policy) = container["HostConfig"]["RestartPolicy"]["Name"].as_str()
            {
                let restart = match restart_policy {
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
                };
                service.insert("restart".to_string(), ComposeValue::String(restart));
            }

            // Network mode
            if let Some(network_mode) = container["HostConfig"]["NetworkMode"].as_str() {
                if network_mode != "default" {
                    service.insert(
                        "network_mode".to_string(),
                        ComposeValue::String(network_mode.to_string()),
                    );
                }
            }

            // Labels
            if let Some(labels) = container["Config"]["Labels"].as_object() {
                let mut label_map = HashMap::new();
                for (key, value) in labels {
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
                    logging.insert(
                        "driver".to_string(),
                        ComposeValue::String(driver.to_string()),
                    );

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

            // User
            if let Some(user) = container["Config"]["User"].as_str() {
                if !user.is_empty() {
                    service.insert("user".to_string(), ComposeValue::String(user.to_string()));
                }
            }

            // Working directory
            if let Some(workdir) = container["Config"]["WorkingDir"].as_str() {
                if !workdir.is_empty() {
                    service.insert(
                        "working_dir".to_string(),
                        ComposeValue::String(workdir.to_string()),
                    );
                }
            }

            // Hostname
            if let Some(hostname) = container["Config"]["Hostname"].as_str() {
                service.insert(
                    "hostname".to_string(),
                    ComposeValue::String(hostname.to_string()),
                );
            }

            // Devices
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

            // Capabilities
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

            // Security Opt
            if let Some(security_opt) = container["HostConfig"]["SecurityOpt"].as_array() {
                let security_options = security_opt
                    .iter()
                    .filter_map(|s| s.as_str())
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                if !security_options.is_empty() {
                    service.insert(
                        "security_opt".to_string(),
                        ComposeValue::Vec(security_options),
                    );
                }
            }

            services.insert(name.to_string(), service);
        }
    }

    let mut output = File::create("docker-compose.yml")?;
    writeln!(output, "version: '3'")?;
    writeln!(output, "services:")?;

    for (name, service) in &services {
        writeln!(output, "  {}:", name)?;
        for (key, value) in service {
            match value {
                ComposeValue::String(s) => writeln!(output, "    {}: {}", key, s)?,
                ComposeValue::Vec(arr) => {
                    writeln!(output, "    {}:", key)?;
                    for item in arr {
                        writeln!(output, "      - {}", item)?;
                    }
                }
                ComposeValue::Map(map) => {
                    writeln!(output, "    {}:", key)?;
                    for (k, v) in map {
                        match v {
                            ComposeValue::String(s) => writeln!(output, "      {}: {}", k, s)?,
                            ComposeValue::Vec(arr) => {
                                writeln!(output, "      {}:", k)?;
                                for item in arr {
                                    writeln!(output, "        - {}", item)?;
                                }
                            }
                            ComposeValue::Map(inner_map) => {
                                writeln!(output, "      {}:", k)?;
                                for (inner_k, inner_v) in inner_map {
                                    if let ComposeValue::String(s) = inner_v {
                                        writeln!(output, "        {}: {}", inner_k, s)?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if !volumes.is_empty() {
        writeln!(output, "volumes:")?;
        for (name, _) in volumes {
            writeln!(output, "  {}:", name)?;
        }
    }

    if !networks.is_empty() {
        writeln!(output, "networks:")?;
        for (name, _) in networks {
            writeln!(output, "  {}:", name)?;
        }
    }

    println!("Docker Compose file generated: docker-compose.yml");

    Ok(())
}
