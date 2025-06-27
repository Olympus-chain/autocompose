/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "autocompose",
    about = "Generate Docker Compose files from running containers",
    version = "2.0.0"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "docker", about = "Export Docker containers to docker-compose")]
    Docker(DockerArgs),
    #[command(name = "podman", about = "Export Podman containers to docker-compose")]
    Podman(PodmanArgs),
    #[command(name = "config", about = "Configuration management")]
    Config(ConfigArgs),
    #[command(name = "validate", about = "Validate generated docker-compose files")]
    Validate(ValidateArgs),
}

#[derive(Parser)]
pub struct DockerArgs {
    #[arg(short, long, default_value = "docker-compose.yml")]
    pub output: PathBuf,

    #[arg(short, long, default_value = "3.9")]
    pub version: String,

    #[arg(short, long, help = "Only include running containers")]
    pub running_only: bool,

    #[arg(short, long, help = "Preview without writing file")]
    pub dry_run: bool,

    #[arg(long, help = "Output format")]
    pub format: Option<OutputFormat>,

    #[arg(long, help = "Filter containers by name pattern")]
    pub filter_name: Option<String>,

    #[arg(long, help = "Filter containers by image pattern")]
    pub filter_image: Option<String>,

    #[arg(long, help = "Exclude containers by name pattern")]
    pub exclude_name: Option<String>,

    #[arg(long, help = "Include system containers")]
    pub include_system: bool,

    #[arg(short, long, help = "Interactive mode for container selection")]
    pub interactive: bool,

    #[arg(long, help = "Generate separate network definitions")]
    pub separate_networks: bool,

    #[arg(long, help = "Generate separate volume definitions")]
    pub separate_volumes: bool,
}

#[derive(Parser)]
pub struct PodmanArgs {
    #[arg(short, long, default_value = "docker-compose.yml")]
    pub output: PathBuf,

    #[arg(short, long, default_value = "3.9")]
    pub version: String,

    #[arg(short, long, help = "Preview without writing file")]
    pub dry_run: bool,

    #[arg(long, help = "Output format")]
    pub format: Option<OutputFormat>,

    #[arg(long, help = "Filter containers by name pattern")]
    pub filter_name: Option<String>,

    #[arg(long, help = "Filter containers by image pattern")]
    pub filter_image: Option<String>,

    #[arg(long, help = "Exclude containers by name pattern")]
    pub exclude_name: Option<String>,

    #[arg(long, help = "Include system containers")]
    pub include_system: bool,

    #[arg(short, long, help = "Interactive mode for container selection")]
    pub interactive: bool,

    #[arg(long, help = "Generate separate network definitions")]
    pub separate_networks: bool,

    #[arg(long, help = "Generate separate volume definitions")]
    pub separate_volumes: bool,
}

#[derive(Parser)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub action: ConfigAction,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    #[command(about = "Show current configuration")]
    Show,
    #[command(about = "Set configuration value")]
    Set {
        #[arg(help = "Configuration key")]
        key: String,
        #[arg(help = "Configuration value")]
        value: String,
    },
    #[command(about = "Reset configuration to defaults")]
    Reset,
    #[command(about = "Initialize configuration file")]
    Init {
        #[arg(short, long, help = "Force overwrite existing config")]
        force: bool,
    },
}

#[derive(Parser)]
pub struct ValidateArgs {
    #[arg(help = "Path to docker-compose.yml file")]
    pub file: PathBuf,

    #[arg(long, help = "Check for best practices")]
    pub check_best_practices: bool,

    #[arg(long, help = "Validate against specific compose version")]
    pub compose_version: Option<String>,

    #[arg(long, help = "Output format for validation results")]
    pub format: Option<OutputFormat>,
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Yaml,
    Json,
    Toml,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub default_output: PathBuf,
    pub default_compose_version: String,
    pub default_format: String,
    pub filters: FilterConfig,
    pub validation: ValidationConfig,
    pub performance: PerformanceConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FilterConfig {
    pub exclude_system_containers: bool,
    pub exclude_patterns: Vec<String>,
    pub include_patterns: Vec<String>,
    pub exclude_labels: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ValidationConfig {
    pub check_best_practices: bool,
    pub warn_on_privileged: bool,
    pub warn_on_host_network: bool,
    pub require_restart_policy: bool,
    pub require_healthcheck: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PerformanceConfig {
    pub parallel_processing: bool,
    pub max_concurrent_containers: usize,
    pub cache_image_info: bool,
    pub cache_duration_minutes: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_output: PathBuf::from("docker-compose.yml"),
            default_compose_version: "3.9".to_string(),
            default_format: "yaml".to_string(),
            filters: FilterConfig {
                exclude_system_containers: true,
                exclude_patterns: vec!["^/k8s_".to_string(), "^/registry_".to_string()],
                include_patterns: vec![],
                exclude_labels: {
                    let mut map = HashMap::new();
                    map.insert("io.kubernetes.container".to_string(), "*".to_string());
                    map
                },
            },
            validation: ValidationConfig {
                check_best_practices: true,
                warn_on_privileged: true,
                warn_on_host_network: true,
                require_restart_policy: false,
                require_healthcheck: false,
            },
            performance: PerformanceConfig {
                parallel_processing: true,
                max_concurrent_containers: 10,
                cache_image_info: true,
                cache_duration_minutes: 60,
            },
        }
    }
}

pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut config_path = dirs::config_dir().ok_or("Unable to determine config directory")?;
    config_path.push("autocompose");
    std::fs::create_dir_all(&config_path)?;
    config_path.push("config.toml");
    Ok(config_path)
}

pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    } else {
        Ok(AppConfig::default())
    }
}

pub fn save_config(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    let content = toml::to_string_pretty(config)?;
    std::fs::write(&config_path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.default_compose_version, "3.9");
        assert_eq!(config.default_format, "yaml");
        assert!(config.filters.exclude_system_containers);
        assert!(config.performance.parallel_processing);
    }

    #[test]
    fn test_config_serialization() {
        let config = AppConfig::default();
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: AppConfig = toml::from_str(&serialized).unwrap();
        assert_eq!(
            config.default_compose_version,
            deserialized.default_compose_version
        );
    }
}
