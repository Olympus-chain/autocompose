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

async fn handle_docker_command(args: autocompose::cli::DockerArgs) -> Result<()> {
    let processor = DockerProcessor::new()?;
    let containers = processor.list_containers(args.running_only).await?;

    if containers.is_empty() {
        println!("No containers found.");
        return Ok(());
    }

    println!("Processing {} containers...", containers.len());
    let (services, networks, volumes) = processor.process_containers_parallel(containers).await?;

    let compose_file = ComposeFile {
        version: args.version.clone(),
        services,
        networks: if networks.is_empty() {
            None
        } else {
            Some(networks)
        },
        volumes: if volumes.is_empty() {
            None
        } else {
            Some(volumes)
        },
    };

    let content = match args.format {
        Some(autocompose::cli::OutputFormat::Json) => {
            serde_json::to_string_pretty(&compose_file)?
        }
        Some(autocompose::cli::OutputFormat::Toml) => toml::to_string_pretty(&compose_file)
            .map_err(|e| {
                AutoComposeError::Io(std::io::Error::other(
                    format!("TOML serialization failed: {}", e),
                ))
            })?,
        _ => serde_yaml::to_string(&compose_file)?,
    };

    if args.dry_run {
        println!("=== DRY RUN - Generated Docker Compose ===");
        println!("{}", content);
        println!("=== END DRY RUN ===");
    } else {
        // Validate output path before writing
        let safe_path = validate_output_path(&args.output)?;
        tokio::fs::write(&safe_path, content).await?;
        println!("Docker Compose file generated: {}", safe_path.display());

        let validator = Validator::new(true, Some(args.version));
        let validation_report = validator.validate_compose_object(&compose_file);

        if !validation_report.warnings.is_empty() || !validation_report.suggestions.is_empty() {
            println!("\n=== Validation Report ===");
            let report_text = format_validation_report(&validation_report, "text")?;
            println!("{}", report_text);
        }
    }

    Ok(())
}

async fn handle_podman_command(args: autocompose::cli::PodmanArgs) -> Result<()> {
    let processor = PodmanProcessor::new();
    let container_ids = processor.list_containers().await?;

    if container_ids.is_empty() {
        println!("No containers found.");
        return Ok(());
    }

    println!("Processing {} containers...", container_ids.len());
    let (services, networks, volumes) =
        processor.process_containers_parallel(container_ids).await?;

    let compose_file = ComposeFile {
        version: args.version.clone(),
        services,
        networks: if networks.is_empty() {
            None
        } else {
            Some(networks)
        },
        volumes: if volumes.is_empty() {
            None
        } else {
            Some(volumes)
        },
    };

    let content = match args.format {
        Some(autocompose::cli::OutputFormat::Json) => {
            serde_json::to_string_pretty(&compose_file)?
        }
        Some(autocompose::cli::OutputFormat::Toml) => toml::to_string_pretty(&compose_file)
            .map_err(|e| {
                AutoComposeError::Io(std::io::Error::other(
                    format!("TOML serialization failed: {}", e),
                ))
            })?,
        _ => serde_yaml::to_string(&compose_file)?,
    };

    if args.dry_run {
        println!("=== DRY RUN - Generated Docker Compose ===");
        println!("{}", content);
        println!("=== END DRY RUN ===");
    } else {
        // Validate output path before writing
        let safe_path = validate_output_path(&args.output)?;
        tokio::fs::write(&safe_path, content).await?;
        println!("Docker Compose file generated: {}", safe_path.display());

        let validator = Validator::new(true, Some(args.version));
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
    let validator = Validator::new(args.check_best_practices, args.compose_version);

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
                "performance.parallel_processing" => {
                    config.performance.parallel_processing = value.parse().unwrap_or(true);
                }
                "performance.max_concurrent_containers" => {
                    config.performance.max_concurrent_containers = value.parse().unwrap_or(10);
                }
                _ => {
                    eprintln!("Unknown configuration key: {}", key);
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
