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
    cli::{load_config, OutputFormat, PodmanArgs},
    podman::PodmanProcessor,
    validation::{format_validation_report, Validator},
    AutoComposeError, ComposeFile, Result,
};
use regex::Regex;
use tokio::process::Command;

#[derive(Parser)]
#[command(
    name = "Podman Compose Exporter",
    about = "Export Podman containers to a docker-compose file"
)]
struct Args {
    #[command(flatten)]
    podman_args: PodmanArgs,
}

#[derive(serde::Deserialize)]
struct PodmanContainer {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Names")]
    names: Vec<String>,
    #[serde(rename = "Image")]
    image: String,
    #[serde(rename = "State")]
    state: String,
}

async fn get_container_list() -> Result<Vec<PodmanContainer>> {
    let output = Command::new("podman")
        .args(&["ps", "-a", "--format", "json"])
        .output()
        .await?;

    if !output.status.success() {
        return Err(AutoComposeError::PodmanCommand(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let stdout = String::from_utf8(output.stdout)?;
    let mut containers = Vec::new();

    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let container: PodmanContainer = serde_json::from_str(line)?;
        containers.push(container);
    }

    Ok(containers)
}

async fn filter_containers(
    containers: Vec<PodmanContainer>,
    args: &PodmanArgs,
) -> Result<Vec<String>> {
    let mut filtered_ids = Vec::new();

    for container in containers {
        let container_name = container
            .names
            .first()
            .map(|name| name.trim_start_matches('/'))
            .unwrap_or("");

        if !args.include_system && is_system_container(container_name, &container.image) {
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
            if !regex.is_match(&container.image) {
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

        filtered_ids.push(container.id);
    }

    Ok(filtered_ids)
}

fn is_system_container(name: &str, image: &str) -> bool {
    let system_patterns = ["k8s_", "registry_", "podman_", "portainer", "watchtower"];

    for pattern in &system_patterns {
        if name.starts_with(pattern) || image.contains(pattern) {
            return true;
        }
    }

    false
}

async fn interactive_container_selection(containers: Vec<PodmanContainer>) -> Result<Vec<String>> {
    let items: Vec<String> = containers
        .iter()
        .map(|container| {
            let name = container
                .names
                .first()
                .map(|name| name.trim_start_matches('/'))
                .unwrap_or("unnamed");
            format!(
                "{} ({}:{}) [{}]",
                name,
                container.image,
                &container.id[..12],
                container.state
            )
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

    let selected_ids: Vec<String> = selections
        .into_iter()
        .map(|index| containers[index].id.clone())
        .collect();

    Ok(selected_ids)
}

async fn write_output(compose_file: &ComposeFile, args: &PodmanArgs) -> Result<()> {
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
    let args = Args::parse().podman_args;

    let _config = load_config().unwrap_or_default();

    let processor = PodmanProcessor::new();

    let containers = get_container_list().await?;

    if containers.is_empty() {
        println!("No containers found.");
        return Ok(());
    }

    let container_ids = if args.interactive {
        interactive_container_selection(containers).await?
    } else {
        filter_containers(containers, &args).await?
    };

    if container_ids.is_empty() {
        println!("No containers selected or matching criteria.");
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
