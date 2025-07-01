/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

use autocompose::{ComposeFile, Service};
use std::collections::HashMap;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_compose_file_serialization_filters_empty_fields() {
        let mut services = HashMap::new();

        // Create a service with some empty fields
        let service = Service {
            image: "nginx:latest".to_string(),
            container_name: Some("test-nginx".to_string()),
            hostname: None,    // Should be omitted
            environment: None, // Empty map should be omitted
            ports: Some(vec!["80:80".to_string()]),
            volumes: None, // Should be omitted
            networks: None,
            network_mode: None,
            dns: None,
            dns_search: None,
            extra_hosts: None,
            restart: Some("unless-stopped".to_string()),
            cap_add: None,
            cap_drop: None,
            security_opt: None,
            deploy: None,
            healthcheck: None,
            labels: None,
            logging: None,
            devices: None,
            user: None,        // Should be omitted (was previously "")
            working_dir: None, // Should be omitted (was previously "")
            entrypoint: None,
            command: None,
            ulimits: None,
            sysctls: None,
            init: None,
            privileged: Some(false),
            tty: None,
            stdin_open: None,
            depends_on: None,
        };

        services.insert("nginx".to_string(), service);

        let compose_file = ComposeFile {
            version: "3.9".to_string(),
            services,
            networks: None,
            volumes: None,
        };

        // Serialize to YAML
        let yaml = serde_yaml::to_string(&compose_file).unwrap();

        // Check that empty fields are not in the output
        assert!(!yaml.contains("hostname:"));
        assert!(!yaml.contains("environment:"));
        assert!(!yaml.contains("user:"));
        assert!(!yaml.contains("working_dir:"));
        assert!(!yaml.contains("volumes:"));
        assert!(!yaml.contains("networks:"));

        // Check that non-empty fields are present
        assert!(yaml.contains("image: nginx:latest"));
        assert!(yaml.contains("container_name: test-nginx"));
        assert!(yaml.contains("ports:"));
        assert!(yaml.contains("restart: unless-stopped"));
        assert!(yaml.contains("privileged: false"));
    }

    #[test]
    fn test_sensitive_env_vars_are_filtered() {
        let mut services = HashMap::new();

        let mut env = HashMap::new();
        env.insert("PATH".to_string(), "/usr/bin".to_string());
        env.insert("DB_PASSWORD".to_string(), "secret123".to_string());
        env.insert("API_KEY".to_string(), "key456".to_string());

        // In real usage, the filtering happens in docker.rs/podman.rs
        // This test verifies the compose file doesn't contain sensitive data
        let filtered_env = autocompose::security::filter_sensitive_env_vars(env);

        let service = Service {
            image: "myapp:latest".to_string(),
            container_name: Some("myapp".to_string()),
            environment: Some(filtered_env),
            hostname: None,
            ports: None,
            volumes: None,
            networks: None,
            network_mode: None,
            dns: None,
            dns_search: None,
            extra_hosts: None,
            restart: None,
            cap_add: None,
            cap_drop: None,
            security_opt: None,
            deploy: None,
            healthcheck: None,
            labels: None,
            logging: None,
            devices: None,
            user: None,
            working_dir: None,
            entrypoint: None,
            command: None,
            ulimits: None,
            sysctls: None,
            init: None,
            privileged: None,
            tty: None,
            stdin_open: None,
            depends_on: None,
        };

        services.insert("myapp".to_string(), service);

        let compose_file = ComposeFile {
            version: "3.9".to_string(),
            services,
            networks: None,
            volumes: None,
        };

        let yaml = serde_yaml::to_string(&compose_file).unwrap();

        // Check that sensitive vars are not in output
        assert!(!yaml.contains("DB_PASSWORD"));
        assert!(!yaml.contains("API_KEY"));
        assert!(!yaml.contains("secret123"));
        assert!(!yaml.contains("key456"));

        // Check that safe vars are present
        assert!(yaml.contains("PATH"));
        assert!(yaml.contains("/usr/bin"));
    }
}
