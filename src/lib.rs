/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

pub mod cli;
pub mod constants;
pub mod docker;
pub mod podman;
pub mod security;
pub mod validation;

pub use docker::ProcessingOptions;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AutoComposeError {
    #[error("Docker connection failed: {0}")]
    DockerConnection(#[from] bollard::errors::Error),
    #[error("Podman command failed: {0}")]
    PodmanCommand(String),
    #[error("JSON parsing failed: {0}")]
    JsonParsing(#[from] serde_json::Error),
    #[error("YAML serialization failed: {0}")]
    YamlSerialization(#[from] serde_yaml::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Container inspection failed: {0}")]
    ContainerInspection(String),
    #[error("Image resolution failed for: {0}")]
    ImageResolution(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, AutoComposeError>;

#[derive(Serialize, Deserialize, Debug)]
pub struct UlimitConfig {
    pub soft: i64,
    pub hard: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ServiceNetworks {
    Simple(Vec<String>),
    Advanced(HashMap<String, NetworkConfig>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<ServiceNetworks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_hosts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_add: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_drop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_opt: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy: Option<Deploy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<HealthCheck>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<HashMap<String, UlimitConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deploy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Resources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<ResourceLimits>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceLimits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Placement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthCheck {
    pub test: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_period: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logging {
    pub driver: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComposeFile {
    pub version: String,
    pub services: HashMap<String, Service>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<HashMap<String, serde_yaml::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<HashMap<String, serde_yaml::Value>>,
}

pub fn normalize_duration_from_ns(nanoseconds: i64) -> String {
    use crate::constants::{NS_PER_SECOND, SECONDS_PER_HOUR, SECONDS_PER_MINUTE};

    let seconds = nanoseconds / NS_PER_SECOND;
    if seconds < SECONDS_PER_MINUTE {
        format!("{}s", seconds)
    } else if seconds < SECONDS_PER_HOUR {
        format!("{}m", seconds / SECONDS_PER_MINUTE)
    } else {
        format!("{}h", seconds / SECONDS_PER_HOUR)
    }
}

pub fn normalize_duration(duration: &str) -> String {
    if duration.ends_with("ns") {
        if let Ok(ns) = duration.trim_end_matches("ns").parse::<i64>() {
            return normalize_duration_from_ns(ns);
        }
    }
    duration.to_string()
}

pub fn sanitize_service_name(name: &str) -> String {
    name.trim_start_matches('/')
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

pub fn filter_system_labels(labels: HashMap<String, String>) -> Option<HashMap<String, String>> {
    let filtered: HashMap<String, String> = labels
        .into_iter()
        .filter(|(key, _)| {
            !key.starts_with("io.buildah")
                && !key.starts_with("org.opencontainers")
                && !key.starts_with("io.podman")
                && !key.starts_with("com.docker")
        })
        .collect();

    if filtered.is_empty() {
        None
    } else {
        Some(filtered)
    }
}

pub trait ContainerProcessor {
    type Container;
    type NetworkInfo;

    fn process_container(
        &self,
        container: Self::Container,
    ) -> Result<(String, Service, Vec<String>, Vec<String>)>;
    fn extract_networks(
        &self,
        network_info: &Self::NetworkInfo,
    ) -> (Vec<String>, HashMap<String, serde_yaml::Value>);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_duration_from_ns() {
        assert_eq!(normalize_duration_from_ns(30_000_000_000), "30s");
        assert_eq!(normalize_duration_from_ns(120_000_000_000), "2m");
        assert_eq!(normalize_duration_from_ns(7200_000_000_000), "2h");
    }

    #[test]
    fn test_normalize_duration() {
        assert_eq!(normalize_duration("30s"), "30s");
        assert_eq!(normalize_duration("30000000000ns"), "30s");
        assert_eq!(normalize_duration("2m"), "2m");
    }

    #[test]
    fn test_sanitize_service_name() {
        assert_eq!(sanitize_service_name("/my-container"), "my-container");
        assert_eq!(sanitize_service_name("my.weird@name"), "my_weird_name");
    }

    #[test]
    fn test_filter_system_labels() {
        let mut labels = HashMap::new();
        labels.insert("io.buildah.version".to_string(), "1.0".to_string());
        labels.insert("custom.label".to_string(), "value".to_string());
        labels.insert(
            "org.opencontainers.image.version".to_string(),
            "1.0".to_string(),
        );

        let filtered = filter_system_labels(labels).unwrap();
        assert_eq!(filtered.len(), 1);
        assert!(filtered.contains_key("custom.label"));
    }
}
