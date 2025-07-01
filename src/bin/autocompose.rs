/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

use clap::Parser;
use autocompose::{
    cli::{get_config_path, load_config, save_config, AppConfig, Cli, Commands, ConfigAction},
    docker::DockerProcessor,
    podman::PodmanProcessor,
    security::validate_output_path,
    validation::{format_validation_report, Validator},
    AutoComposeError, ComposeFile, Result,
};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use std::collections::HashMap;

fn matches_pattern(text: &str, pattern: &str) -> bool {
    // Convert wildcard pattern to regex
    let regex_pattern = pattern
        .replace("*", ".*")
        .replace("?", ".");
    
    regex::Regex::new(&format!("^{}$", regex_pattern))
        .map(|re| re.is_match(text))
        .unwrap_or(false)
}

fn should_include_container_docker(
    name: &str,
    service: &autocompose::Service,
    args: &autocompose::cli::DockerArgs,
    config: &autocompose::cli::AppConfig,
) -> bool {
    // If specific containers are requested, only include those
    if !args.containers.is_empty() {
        return args.containers.iter().any(|c| {
            name.contains(c) || 
            service.container_name.as_ref().map_or(false, |cn| cn.contains(c))
        });
    }
    
    let mut should_include = true;
    
    // Apply --filter patterns first
    if let Some(filters) = &args.filter {
        should_include = filters.iter().any(|pattern| {
            matches_pattern(name, pattern) ||
            service.container_name.as_ref().map_or(false, |cn| matches_pattern(cn, pattern)) ||
            matches_pattern(&service.image, pattern)
        });
    }
    
    // Apply specific filters if no general filter
    if should_include && args.filter.is_none() {
        // Check filter_name and filter_image
        if let Some(filter_name) = &args.filter_name {
            should_include = name.contains(filter_name) || 
                service.container_name.as_ref().map_or(false, |cn| cn.contains(filter_name));
        }
        if should_include && args.filter_image.is_some() {
            if let Some(filter_image) = &args.filter_image {
                should_include = service.image.contains(filter_image);
            }
        }
    }
    
    // Apply --exclude patterns
    if should_include {
        if let Some(excludes) = &args.exclude {
            for pattern in excludes {
                if matches_pattern(name, pattern) ||
                   service.container_name.as_ref().map_or(false, |cn| matches_pattern(cn, pattern)) ||
                   matches_pattern(&service.image, pattern) {
                    should_include = false;
                    break;
                }
            }
        }
    }
    
    // Apply exclude_system logic
    if should_include && (args.exclude_system || (!args.include_system && config.filters.exclude_system_containers)) {
        // Apply exclude patterns from config
        for pattern in &config.filters.exclude_patterns {
            if name.contains(pattern) {
                should_include = false;
                break;
            }
        }
    }
    
    // Apply include patterns from config if any
    if should_include && !config.filters.include_patterns.is_empty() {
        let matches_include = config.filters.include_patterns.iter().any(|pattern| {
            name.contains(pattern)
        });
        if !matches_include {
            should_include = false;
        }
    }
    
    should_include
}

fn should_include_container_podman(
    name: &str,
    service: &autocompose::Service,
    args: &autocompose::cli::PodmanArgs,
    config: &autocompose::cli::AppConfig,
) -> bool {
    // If specific containers are requested, only include those
    if !args.containers.is_empty() {
        return args.containers.iter().any(|c| {
            name.contains(c) || 
            service.container_name.as_ref().map_or(false, |cn| cn.contains(c))
        });
    }
    
    let mut should_include = true;
    
    // Apply --filter patterns first
    if let Some(filters) = &args.filter {
        should_include = filters.iter().any(|pattern| {
            matches_pattern(name, pattern) ||
            service.container_name.as_ref().map_or(false, |cn| matches_pattern(cn, pattern)) ||
            matches_pattern(&service.image, pattern)
        });
    }
    
    // Apply specific filters if no general filter
    if should_include && args.filter.is_none() {
        // Check filter_name and filter_image
        if let Some(filter_name) = &args.filter_name {
            should_include = name.contains(filter_name) || 
                service.container_name.as_ref().map_or(false, |cn| cn.contains(filter_name));
        }
        if should_include && args.filter_image.is_some() {
            if let Some(filter_image) = &args.filter_image {
                should_include = service.image.contains(filter_image);
            }
        }
    }
    
    // Apply --exclude patterns
    if should_include {
        if let Some(excludes) = &args.exclude {
            for pattern in excludes {
                if matches_pattern(name, pattern) ||
                   service.container_name.as_ref().map_or(false, |cn| matches_pattern(cn, pattern)) ||
                   matches_pattern(&service.image, pattern) {
                    should_include = false;
                    break;
                }
            }
        }
    }
    
    // Apply exclude_system logic
    if should_include && (args.exclude_system || (!args.include_system && config.filters.exclude_system_containers)) {
        // Apply exclude patterns from config
        for pattern in &config.filters.exclude_patterns {
            if name.contains(pattern) {
                should_include = false;
                break;
            }
        }
    }
    
    // Apply include patterns from config if any
    if should_include && !config.filters.include_patterns.is_empty() {
        let matches_include = config.filters.include_patterns.iter().any(|pattern| {
            name.contains(pattern)
        });
        if !matches_include {
            should_include = false;
        }
    }
    
    should_include
}

fn format_compose_output(
    compose_file: &ComposeFile,
    format: Option<autocompose::cli::OutputFormat>,
    compact: bool,
) -> Result<String> {
    match format {
        Some(autocompose::cli::OutputFormat::Json) => {
            if compact {
                Ok(serde_json::to_string(&compose_file)?)
            } else {
                Ok(serde_json::to_string_pretty(&compose_file)?)
            }
        }
        Some(autocompose::cli::OutputFormat::Toml) => {
            let result = if compact {
                toml::to_string(&compose_file)
            } else {
                toml::to_string_pretty(&compose_file)
            };
            result.map_err(|e| {
                AutoComposeError::Io(std::io::Error::other(
                    format!("TOML serialization failed: {}", e),
                ))
            })
        }
        _ => {
            if compact {
                // For YAML compact mode, use minimal formatting
                Ok(serde_yaml::to_string(&compose_file)?)
            } else {
                Ok(serde_yaml::to_string(&compose_file)?)
            }
        }
    }
}

fn interactive_container_selection<T: Clone>(
    containers: Vec<T>,
    get_display_info: impl Fn(&T) -> (String, String, String, String),
) -> Result<Vec<T>> {
    let items: Vec<String> = containers
        .iter()
        .map(|container| {
            let (name, image, id, state) = get_display_info(container);
            let id_display = if id.len() > 12 { &id[..12] } else { &id };
            format!("{} ({}:{}) [{}]", name, image, id_display, state)
        })
        .collect();

    if items.is_empty() {
        println!("No containers found to select from.");
        return Ok(vec![]);
    }

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select containers to include in docker-compose:")
        .items(&items)
        .defaults(&vec![true; items.len()])
        .interact()
        .map_err(|e| {
            AutoComposeError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Interactive selection failed: {}", e),
            ))
        })?;

    let selected_containers: Vec<T> = selections
        .into_iter()
        .map(|index| containers[index].clone())
        .collect();

    Ok(selected_containers)
}

async fn handle_docker_command(mut args: autocompose::cli::DockerArgs) -> Result<()> {
    // Load configuration and apply defaults
    let config = load_config().unwrap_or_default();
    
    // Enable debug output if requested
    let debug_enabled = args.debug || args.verbose > 0;
    if debug_enabled {
        let level = match args.verbose {
            0 if args.debug => "DEBUG",
            1 => "INFO",
            2 => "DEBUG",
            _ => "TRACE",
        };
        eprintln!("[{}] Starting autocompose docker command", level);
        eprintln!("[{}] Configuration loaded from: {:?}", level, get_config_path().ok());
    }
    
    // Apply config defaults if CLI args not provided
    if args.compose_version == "3.9" { // Check if it's the CLI default
        args.compose_version = config.default_compose_version.clone();
    }
    if args.output == std::path::PathBuf::from("docker-compose.yml") {
        args.output = config.default_output.clone();
    }
    if args.format.is_none() {
        args.format = match config.default_format.as_str() {
            "json" => Some(autocompose::cli::OutputFormat::Json),
            "toml" => Some(autocompose::cli::OutputFormat::Toml),
            _ => Some(autocompose::cli::OutputFormat::Yaml),
        };
    }
    
    // Handle Docker connection options
    let processor = if let Some(docker_host) = &args.docker_host {
        DockerProcessor::new_with_host(docker_host)?
    } else if let Some(context) = &args.context {
        DockerProcessor::new_with_context(context)?
    } else {
        DockerProcessor::new()?
    };
    // If --all is specified, include all containers regardless of running_only
    let include_all = args.all || !args.running_only;
    if debug_enabled {
        eprintln!("[DEBUG] Fetching containers (include_all: {})", include_all);
    }
    let mut containers = processor.list_containers(!include_all).await?;
    if debug_enabled {
        eprintln!("[DEBUG] Found {} containers", containers.len());
    }

    // Apply state filter if specified
    if let Some(state_filter) = &args.state {
        containers.retain(|c| {
            c.state.as_ref()
                .map(|s| format!("{:?}", s).to_lowercase().contains(&state_filter.to_lowercase()))
                .unwrap_or(false)
        });
    }
    
    // Apply label filters
    if let Some(label_filters) = &args.label_filter {
        containers.retain(|c| {
            if let Some(labels) = &c.labels {
                label_filters.iter().all(|filter| {
                    if let Some((key, value)) = filter.split_once('=') {
                        labels.get(key).map_or(false, |v| v == value)
                    } else {
                        labels.contains_key(filter)
                    }
                })
            } else {
                false
            }
        });
    }
    
    // Apply has-label filter
    if let Some(has_label) = &args.has_label {
        containers.retain(|c| {
            c.labels.as_ref()
                .map(|labels| labels.contains_key(has_label))
                .unwrap_or(false)
        });
    }

    if containers.is_empty() {
        println!("No containers found matching the specified filters.");
        return Ok(());
    }

    // Interactive mode
    if args.interactive {
        containers = interactive_container_selection(containers, |container| {
            let name = container
                .names
                .as_ref()
                .and_then(|names| names.first())
                .map(|n| n.trim_start_matches('/'))
                .unwrap_or("unnamed")
                .to_string();
            let image = container.image.clone().unwrap_or_else(|| "unknown".to_string());
            let id = container.id.clone().unwrap_or_else(|| "unknown".to_string());
            let state = container
                .state
                .as_ref()
                .map(|s| format!("{:?}", s))
                .unwrap_or_else(|| "unknown".to_string());
            (name, image, id, state)
        })?;
        
        if containers.is_empty() {
            println!("No containers selected.");
            return Ok(());
        }
    }

    println!("Processing {} containers...", containers.len());
    let (services, networks, volumes) = if args.include_sensitive {
        processor.process_containers_parallel_with_options(
            containers, 
            autocompose::docker::ProcessingOptions { include_sensitive: true }
        ).await?
    } else {
        processor.process_containers_parallel(containers).await?
    };

    // Apply filters from config and command line
    let mut filtered_services = HashMap::new();
    for (name, mut service) in services {
        // Use the comprehensive filtering function
        let should_include = should_include_container_docker(&name, &service, &args, &config);
        if debug_enabled && !should_include {
            eprintln!("[DEBUG] Filtered out container: {}", name);
        }
        
        // Add health checks if requested
        if args.add_healthchecks && service.healthcheck.is_none() {
            service.healthcheck = Some(autocompose::HealthCheck {
                test: vec!["CMD".to_string(), "curl".to_string(), "-f".to_string(), "http://localhost/".to_string()],
                interval: Some("30s".to_string()),
                timeout: Some("10s".to_string()),
                retries: Some(3),
                start_period: Some("40s".to_string()),
            });
        }
        
        // Add resource limits if requested
        if args.resource_limits && service.deploy.is_none() {
            service.deploy = Some(autocompose::Deploy {
                resources: Some(autocompose::Resources {
                    limits: Some(autocompose::ResourceLimits {
                        cpus: Some("0.5".to_string()),
                        memory: Some("512M".to_string()),
                    }),
                }),
                placement: None,
            });
        }
        
        if should_include {
            filtered_services.insert(name, service);
        }
    }
    
    let compose_file = ComposeFile {
        version: args.compose_version.clone(),
        services: filtered_services,
        networks: if networks.is_empty() || (!args.include_networks && !args.separate_networks) {
            None
        } else {
            Some(networks)
        },
        volumes: if volumes.is_empty() || (!args.include_volumes && !args.separate_volumes) {
            None
        } else {
            Some(volumes)
        },
    };

    let content = format_compose_output(&compose_file, args.format.clone(), args.compact)?;

    if args.dry_run || args.preview {
        println!("=== DRY RUN - Generated Docker Compose ===");
        println!("{}", content);
        println!("=== END DRY RUN ===");
    } else {
        // Validate output path before writing
        let safe_path = validate_output_path(&args.output)?;
        tokio::fs::write(&safe_path, content).await?;
        println!("Docker Compose file generated: {}", safe_path.display());

        let validator = Validator::new(config.validation.check_best_practices, Some(args.compose_version));
        let validation_report = validator.validate_compose_object(&compose_file);

        if !validation_report.warnings.is_empty() || !validation_report.suggestions.is_empty() {
            println!("\n=== Validation Report ===");
            let report_text = format_validation_report(&validation_report, "text")?;
            println!("{}", report_text);
        }
    }

    Ok(())
}

async fn handle_podman_command(mut args: autocompose::cli::PodmanArgs) -> Result<()> {
    // Load configuration and apply defaults
    let config = load_config().unwrap_or_default();
    
    // Apply config defaults if CLI args not provided
    if args.compose_version == "3.9" { // Check if it's the CLI default
        args.compose_version = config.default_compose_version.clone();
    }
    if args.output == std::path::PathBuf::from("docker-compose.yml") {
        args.output = config.default_output.clone();
    }
    if args.format.is_none() {
        args.format = match config.default_format.as_str() {
            "json" => Some(autocompose::cli::OutputFormat::Json),
            "toml" => Some(autocompose::cli::OutputFormat::Toml),
            _ => Some(autocompose::cli::OutputFormat::Yaml),
        };
    }
    
    let processor = PodmanProcessor::new();
    let mut container_ids = processor.list_containers().await?;

    if container_ids.is_empty() {
        println!("No containers found.");
        return Ok(());
    }
    
    // Apply Podman-specific options
    if args.include_pods {
        // Add pod IDs to the list
        if let Ok(output) = tokio::process::Command::new("podman")
            .args(["pod", "ps", "-q"])
            .output()
            .await
        {
            let pod_ids = String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            container_ids.extend(pod_ids);
        }
    }

    // Interactive mode for Podman
    if args.interactive {
        // Get container details for display
        let mut container_infos = Vec::new();
        for id in &container_ids {
            if let Ok(output) = tokio::process::Command::new("podman")
                .args(["inspect", id])
                .output()
                .await
            {
                if let Ok(json) = serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout) {
                    if let Some(container) = json.first() {
                        let name = container["Name"]
                            .as_str()
                            .unwrap_or("unnamed")
                            .trim_start_matches('/');
                        let image = container["Config"]["Image"]
                            .as_str()
                            .unwrap_or("unknown");
                        let state = container["State"]["Status"]
                            .as_str()
                            .unwrap_or("unknown");
                        container_infos.push((id.clone(), name.to_string(), image.to_string(), state.to_string()));
                    }
                }
            }
        }

        let selected = interactive_container_selection(container_infos.clone(), |info| {
            (info.1.clone(), info.2.clone(), info.0.clone(), info.3.clone())
        })?;
        
        container_ids = selected.into_iter().map(|info| info.0).collect();
        
        if container_ids.is_empty() {
            println!("No containers selected.");
            return Ok(());
        }
    }

    println!("Processing {} containers...", container_ids.len());
    let (services, networks, volumes) =
        processor.process_containers_parallel(container_ids).await?;

    // Apply filters from config and command line
    let mut filtered_services = HashMap::new();
    for (name, mut service) in services {
        // Use the comprehensive filtering function
        let should_include = should_include_container_podman(&name, &service, &args, &config);
        
        // Add health checks if requested
        if args.add_healthchecks && service.healthcheck.is_none() {
            service.healthcheck = Some(autocompose::HealthCheck {
                test: vec!["CMD".to_string(), "curl".to_string(), "-f".to_string(), "http://localhost/".to_string()],
                interval: Some("30s".to_string()),
                timeout: Some("10s".to_string()),
                retries: Some(3),
                start_period: Some("40s".to_string()),
            });
        }
        
        // Add resource limits if requested
        if args.resource_limits && service.deploy.is_none() {
            service.deploy = Some(autocompose::Deploy {
                resources: Some(autocompose::Resources {
                    limits: Some(autocompose::ResourceLimits {
                        cpus: Some("0.5".to_string()),
                        memory: Some("512M".to_string()),
                    }),
                }),
                placement: None,
            });
        }
        
        if should_include {
            filtered_services.insert(name, service);
        }
    }
    
    let compose_file = ComposeFile {
        version: args.compose_version.clone(),
        services: filtered_services,
        networks: if networks.is_empty() || (!args.include_networks && !args.separate_networks) {
            None
        } else {
            Some(networks)
        },
        volumes: if volumes.is_empty() || (!args.include_volumes && !args.separate_volumes) {
            None
        } else {
            Some(volumes)
        },
    };

    let content = format_compose_output(&compose_file, args.format.clone(), args.compact)?;

    if args.dry_run || args.preview {
        println!("=== DRY RUN - Generated Docker Compose ===");
        println!("{}", content);
        println!("=== END DRY RUN ===");
    } else {
        // Validate output path before writing
        let safe_path = validate_output_path(&args.output)?;
        tokio::fs::write(&safe_path, content).await?;
        println!("Docker Compose file generated: {}", safe_path.display());

        let validator = Validator::new(config.validation.check_best_practices, Some(args.compose_version));
        let validation_report = validator.validate_compose_object(&compose_file);

        if !validation_report.warnings.is_empty() || !validation_report.suggestions.is_empty() {
            println!("\n=== Validation Report ===");
            let report_text = format_validation_report(&validation_report, "text")?;
            println!("{}", report_text);
        }
    }

    Ok(())
}

async fn handle_validate_command(args: autocompose::cli::ValidateArgs) -> Result<()> {
    // In strict mode, always check best practices
    let check_best_practices = args.strict || args.check_best_practices;
    let validator = Validator::new(check_best_practices, args.compose_version);

    let report = validator.validate_file(&args.file).await?;

    let format = args
        .format
        .map(|f| match f {
            autocompose::cli::OutputFormat::Json => "json",
            autocompose::cli::OutputFormat::Yaml => "yaml",
            autocompose::cli::OutputFormat::Toml => "toml",
        })
        .unwrap_or("text");

    let output = format_validation_report(&report, format)?;
    println!("{}", output);

    // In strict mode, fail if there are any warnings or suggestions
    if args.strict {
        if !report.warnings.is_empty() || !report.suggestions.is_empty() {
            eprintln!("\n❌ Validation failed in strict mode due to warnings or suggestions");
            std::process::exit(1);
        }
    }

    if !report.is_valid {
        std::process::exit(1);
    }

    Ok(())
}

fn handle_config_command(args: autocompose::cli::ConfigArgs) -> Result<()> {
    match args.action {
        ConfigAction::Show => {
            let config = load_config().unwrap_or_default();
            let config_str = toml::to_string_pretty(&config).map_err(|e| {
                AutoComposeError::Io(std::io::Error::other(
                    format!("Config serialization failed: {}", e),
                ))
            })?;
            println!("Current configuration:");
            println!("{}", config_str);
        }
        ConfigAction::Set { key, value } => {
            let mut config = load_config().unwrap_or_default();
            let value_clone = value.clone();

            match key.as_str() {
                "default_output" => config.default_output = value.into(),
                "default_compose_version" => config.default_compose_version = value,
                "default_format" => config.default_format = value,
                "filters.exclude_system_containers" => {
                    config.filters.exclude_system_containers = value.parse().unwrap_or(true);
                }
                "validation.check_best_practices" => {
                    config.validation.check_best_practices = value.parse().unwrap_or(true);
                }
                "validation.warn_on_privileged" => {
                    config.validation.warn_on_privileged = value.parse().unwrap_or(true);
                }
                "validation.warn_on_host_network" => {
                    config.validation.warn_on_host_network = value.parse().unwrap_or(true);
                }
                "validation.require_restart_policy" => {
                    config.validation.require_restart_policy = value.parse().unwrap_or(false);
                }
                "validation.require_healthcheck" => {
                    config.validation.require_healthcheck = value.parse().unwrap_or(false);
                }
                "performance.parallel_processing" => {
                    config.performance.parallel_processing = value.parse().unwrap_or(true);
                }
                "performance.max_concurrent_containers" => {
                    config.performance.max_concurrent_containers = value.parse().unwrap_or(10);
                }
                "performance.cache_image_info" => {
                    config.performance.cache_image_info = value.parse().unwrap_or(true);
                }
                "performance.cache_duration_minutes" => {
                    config.performance.cache_duration_minutes = value.parse().unwrap_or(60);
                }
                _ => {
                    eprintln!("Unknown configuration key: {}", key);
                    
                    // Suggest similar keys
                    let suggestions = match key.as_str() {
                        "require_healthcheck" => Some("validation.require_healthcheck"),
                        "require_restart_policy" => Some("validation.require_restart_policy"),
                        "check_best_practices" => Some("validation.check_best_practices"),
                        "warn_on_privileged" => Some("validation.warn_on_privileged"),
                        "warn_on_host_network" => Some("validation.warn_on_host_network"),
                        "exclude_system_containers" => Some("filters.exclude_system_containers"),
                        "parallel_processing" => Some("performance.parallel_processing"),
                        "max_concurrent_containers" => Some("performance.max_concurrent_containers"),
                        "cache_image_info" => Some("performance.cache_image_info"),
                        "cache_duration_minutes" => Some("performance.cache_duration_minutes"),
                        _ => None,
                    };
                    
                    if let Some(suggestion) = suggestions {
                        eprintln!("Did you mean: {}", suggestion);
                    }
                    
                    eprintln!("\nAvailable configuration keys:");
                    eprintln!("  default_output");
                    eprintln!("  default_compose_version");
                    eprintln!("  default_format");
                    eprintln!("  filters.exclude_system_containers");
                    eprintln!("  validation.check_best_practices");
                    eprintln!("  validation.warn_on_privileged");
                    eprintln!("  validation.warn_on_host_network");
                    eprintln!("  validation.require_restart_policy");
                    eprintln!("  validation.require_healthcheck");
                    eprintln!("  performance.parallel_processing");
                    eprintln!("  performance.max_concurrent_containers");
                    eprintln!("  performance.cache_image_info");
                    eprintln!("  performance.cache_duration_minutes");
                    
                    return Ok(());
                }
            }

            save_config(&config).map_err(|e| {
                AutoComposeError::Io(std::io::Error::other(
                    format!("Failed to save config: {}", e),
                ))
            })?;
            println!("Configuration updated: {} = {}", key, value_clone);
        }
        ConfigAction::Reset => {
            let config = AppConfig::default();
            save_config(&config).map_err(|e| {
                AutoComposeError::Io(std::io::Error::other(
                    format!("Failed to save config: {}", e),
                ))
            })?;
            println!("Configuration reset to defaults");
        }
        ConfigAction::Init { force } => {
            let config_path = get_config_path().map_err(|e| {
                AutoComposeError::Io(std::io::Error::other(
                    format!("Failed to get config path: {}", e),
                ))
            })?;

            if config_path.exists() && !force {
                eprintln!(
                    "Configuration file already exists at {}. Use --force to overwrite.",
                    config_path.display()
                );
                return Ok(());
            }

            let config = AppConfig::default();
            save_config(&config).map_err(|e| {
                AutoComposeError::Io(std::io::Error::other(
                    format!("Failed to save config: {}", e),
                ))
            })?;
            println!("Configuration file created at {}", config_path.display());
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Docker(args) => handle_docker_command(args).await?,
        Commands::Podman(args) => handle_podman_command(args).await?,
        Commands::Validate(args) => handle_validate_command(args).await?,
        Commands::Config(args) => handle_config_command(args)?,
    }

    Ok(())
}
