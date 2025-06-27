/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use autocompose::{
    cli::{load_config, DockerArgs, OutputFormat},
    docker::DockerProcessor,
    validation::{format_validation_report, Validator},
    AutoComposeError, ComposeFile, Result,
};
use regex::Regex;

#[derive(Parser)]
#[command(
    name = "Docker Compose Exporter",
    about = "Export Docker containers to a docker-compose file"
)]
struct Args {
    #[command(flatten)]
    docker_args: DockerArgs,
}

async fn filter_containers(
    processor: &DockerProcessor,
    args: &DockerArgs,
) -> Result<Vec<bollard::models::ContainerSummary>> {
    let containers = processor.list_containers(args.running_only).await?;

    let mut filtered_containers = Vec::new();

    for container in containers {
        let empty_names = vec![];
        let container_names = container.names.as_ref().unwrap_or(&empty_names);
        let container_image = container.image.as_deref().unwrap_or("");

        let container_name = container_names
            .first()
            .map(|name| name.trim_start_matches('/'))
            .unwrap_or("");

        if !args.include_system && is_system_container(container_name, container_image) {
            continue;
        }

        if let Some(ref filter_name) = args.filter_name {
            let regex = Regex::new(filter_name).map_err(|e| {
                AutoComposeError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid name filter regex: {}", e),
                ))
            })?;
            if !regex.is_match(container_name) {
                continue;
            }
        }

        if let Some(ref filter_image) = args.filter_image {
            let regex = Regex::new(filter_image).map_err(|e| {
                AutoComposeError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid image filter regex: {}", e),
                ))
            })?;
            if !regex.is_match(container_image) {
                continue;
            }
        }

        if let Some(ref exclude_name) = args.exclude_name {
            let regex = Regex::new(exclude_name).map_err(|e| {
                AutoComposeError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid exclude name regex: {}", e),
                ))
            })?;
            if regex.is_match(container_name) {
                continue;
            }
        }

        filtered_containers.push(container);
    }

    Ok(filtered_containers)
}

fn is_system_container(name: &str, image: &str) -> bool {
    let system_patterns = ["k8s_", "registry_", "docker_", "portainer", "watchtower"];

    for pattern in &system_patterns {
        if name.starts_with(pattern) || image.contains(pattern) {
            return true;
        }
    }

    false
}

async fn interactive_container_selection(
    containers: Vec<bollard::models::ContainerSummary>,
) -> Result<Vec<bollard::models::ContainerSummary>> {
    let items: Vec<String> = containers
        .iter()
        .map(|container| {
            let name = container
                .names
                .as_ref()
                .and_then(|names| names.first())
                .map(|name| name.trim_start_matches('/'))
                .unwrap_or("unnamed");
            let image = container.image.as_deref().unwrap_or("unknown");
            let state = container
                .state
                .as_ref()
                .map(|s| format!("{:?}", s))
                .unwrap_or_else(|| "unknown".to_string());
            let id_short = container.id.as_deref().unwrap_or("unknown");
            let id_display = if id_short.len() >= 12 {
                &id_short[..12]
            } else {
                id_short
            };
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

    let selected_containers: Vec<bollard::models::ContainerSummary> = selections
        .into_iter()
        .map(|index| containers[index].clone())
        .collect();

    Ok(selected_containers)
}

async fn write_output(compose_file: &ComposeFile, args: &DockerArgs) -> Result<()> {
    let content = match args.format {
        Some(OutputFormat::Json) => serde_json::to_string_pretty(compose_file)?,
        Some(OutputFormat::Toml) => toml::to_string_pretty(compose_file).map_err(|e| {
            AutoComposeError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("TOML serialization failed: {}", e),
            ))
        })?,
        _ => serde_yaml::to_string(compose_file)?,
    };

    if args.dry_run {
        println!("=== DRY RUN - Generated Docker Compose ===");
        println!("{}", content);
        println!("=== END DRY RUN ===");
    } else {
        tokio::fs::write(&args.output, content).await?;
        println!("Docker Compose file generated: {}", args.output.display());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse().docker_args;

    let _config = load_config().unwrap_or_default();

    let processor = DockerProcessor::new()?;

    let mut containers = filter_containers(&processor, &args).await?;

    if containers.is_empty() {
        println!("No containers found matching the specified criteria.");
        return Ok(());
    }

    if args.interactive {
        containers = interactive_container_selection(containers).await?;
        if containers.is_empty() {
            println!("No containers selected.");
            return Ok(());
        }
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

    if !args.dry_run {
        let validator = Validator::new(true, Some(args.version.clone()));
        let validation_report = validator.validate_compose_object(&compose_file);

        if !validation_report.warnings.is_empty() || !validation_report.suggestions.is_empty() {
            println!("\n=== Validation Report ===");
            let report_text = format_validation_report(&validation_report, "text")?;
            println!("{}", report_text);
        }
    }

    write_output(&compose_file, &args).await?;

    Ok(())
}
