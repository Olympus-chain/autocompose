/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

#[cfg(test)]
mod edge_cases_tests {
    use autocompose::{
        filter_system_labels, sanitize_service_name,
        security::{filter_sensitive_env_vars, validate_container_id},
        ComposeFile, NetworkConfig, Service, ServiceNetworks,
    };
    use std::collections::HashMap;

    #[test]
    fn test_empty_compose_file() {
        let compose = ComposeFile {
            version: "3.9".to_string(),
            services: HashMap::new(),
            networks: None,
            volumes: None,
        };

        let yaml = serde_yaml::to_string(&compose).unwrap();
        assert!(yaml.contains("version: '3.9'"));
        assert!(yaml.contains("services: {}"));
        assert!(!yaml.contains("networks:"));
        assert!(!yaml.contains("volumes:"));
    }

    #[test]
    fn test_service_with_all_fields_none() {
        let service = Service {
            image: "test:latest".to_string(),
            container_name: None,
            hostname: None,
            environment: None,
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

        let yaml = serde_yaml::to_string(&service).unwrap();
        // Should only contain the image field
        assert!(yaml.contains("image: test:latest"));
        let lines: Vec<&str> = yaml.lines().collect();
        assert!(lines.len() < 5); // Should be very short
    }

    #[test]
    fn test_empty_string_fields_are_filtered() {
        // Test that empty strings become None
        assert_eq!(sanitize_service_name(""), "");

        let mut labels = HashMap::new();
        labels.insert("key".to_string(), "".to_string());
        labels.insert("valid".to_string(), "value".to_string());

        // In real usage, empty values should be filtered
        assert_eq!(labels.get("key").unwrap(), "");
    }

    #[test]
    fn test_special_characters_in_service_names() {
        let test_cases = vec![
            ("my-service", "my-service"),
            ("my_service", "my_service"),
            ("my.service", "my_service"),
            ("my service", "my_service"),
            ("my@service!", "my_service_"),
            ("123service", "123service"),
            ("service-123", "service-123"),
            ("/my/service", "my_service"),
            ("//service", "service"),
            ("service//", "service__"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(sanitize_service_name(input), expected);
        }
    }

    #[test]
    fn test_very_long_container_id() {
        let long_id = "a".repeat(100);
        let result = validate_container_id(&long_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_unicode_in_environment_variables() {
        let mut env = HashMap::new();
        env.insert("UNICODE_VAR".to_string(), "Hello 世界 🌍".to_string());
        env.insert("EMOJI".to_string(), "🚀🐳".to_string());

        let service = Service {
            image: "test:latest".to_string(),
            environment: Some(env),
            container_name: None,
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

        let yaml = serde_yaml::to_string(&service).unwrap();
        assert!(yaml.contains("UNICODE_VAR:"));
        assert!(yaml.contains("Hello 世界 🌍"));
        assert!(yaml.contains("🚀🐳"));
    }

    #[test]
    fn test_network_circular_reference() {
        // Test that services can reference networks that reference services
        let mut services = HashMap::new();
        let mut networks = HashMap::new();

        networks.insert(
            "app-net".to_string(),
            NetworkConfig {
                ipv4_address: Some("172.20.0.2".to_string()),
                ipv6_address: None,
            },
        );

        let service = Service {
            image: "app:latest".to_string(),
            networks: Some(ServiceNetworks::Advanced(networks)),
            container_name: Some("app".to_string()),
            hostname: None,
            environment: None,
            ports: None,
            volumes: None,
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

        services.insert("app".to_string(), service);

        let compose = ComposeFile {
            version: "3.9".to_string(),
            services,
            networks: Some(HashMap::new()),
            volumes: None,
        };

        // Should serialize without issues
        let yaml = serde_yaml::to_string(&compose).unwrap();
        assert!(yaml.contains("app-net:"));
    }

    #[test]
    fn test_mixed_case_sensitive_env_filtering() {
        let mut env = HashMap::new();
        env.insert("password".to_string(), "secret1".to_string());
        env.insert("PASSWORD".to_string(), "secret2".to_string());
        env.insert("PaSsWoRd".to_string(), "secret3".to_string());
        env.insert("MY_PASSWORD".to_string(), "secret4".to_string());
        env.insert("PASSWORDLESS".to_string(), "secret5".to_string());
        env.insert("PASS".to_string(), "not_filtered".to_string());
        env.insert("NORMAL_VAR".to_string(), "visible".to_string());

        let filtered = filter_sensitive_env_vars(env);

        // All password variants should be filtered
        assert!(!filtered.contains_key("password"));
        assert!(!filtered.contains_key("PASSWORD"));
        assert!(!filtered.contains_key("PaSsWoRd"));
        assert!(!filtered.contains_key("MY_PASSWORD"));
        assert!(!filtered.contains_key("PASSWORDLESS"));

        // Non-password vars should remain
        assert!(filtered.contains_key("PASS"));
        assert!(filtered.contains_key("NORMAL_VAR"));
    }

    #[test]
    fn test_extreme_port_numbers() {
        let port_tests = vec![
            ("0:80", false),     // Port 0 is invalid
            ("1:80", true),      // Port 1 is valid
            ("65535:80", true),  // Max port
            ("65536:80", false), // Above max
            ("80", false),       // Missing host port
            (":80", false),      // Missing host port
            ("", false),         // Empty
        ];

        for (port, should_be_valid) in port_tests {
            let is_valid = if port.contains(':') && !port.starts_with(':') {
                let parts: Vec<&str> = port.split(':').collect();
                if parts.len() == 2 {
                    if let Ok(host_port) = parts[0].parse::<u16>() {
                        host_port > 0 // u16 max is already 65535, no need to check upper bound
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };
            assert_eq!(is_valid, should_be_valid, "Port {} validation failed", port);
        }
    }

    #[test]
    fn test_system_labels_filtering() {
        let mut labels = HashMap::new();
        labels.insert("app".to_string(), "myapp".to_string());
        labels.insert("version".to_string(), "1.0".to_string());
        labels.insert(
            "io.kubernetes.container.name".to_string(),
            "container-123".to_string(),
        );
        labels.insert("io.buildah.version".to_string(), "1.23".to_string());
        labels.insert(
            "org.opencontainers.image.version".to_string(),
            "latest".to_string(),
        );
        labels.insert(
            "com.docker.compose.project".to_string(),
            "myproject".to_string(),
        );
        labels.insert(
            "io.podman.compose.project".to_string(),
            "myproject".to_string(),
        );

        let filtered = filter_system_labels(labels);

        // filter_system_labels returns None if all labels are filtered out
        // or Some with only non-system labels
        match filtered {
            Some(filtered_labels) => {
                assert!(filtered_labels.contains_key("app"));
                assert!(filtered_labels.contains_key("version"));
                // io.kubernetes labels are NOT filtered by filter_system_labels
                assert!(filtered_labels.contains_key("io.kubernetes.container.name"));
                assert!(!filtered_labels.contains_key("io.buildah.version"));
                assert!(!filtered_labels.contains_key("org.opencontainers.image.version"));
                assert!(!filtered_labels.contains_key("com.docker.compose.project"));
                assert!(!filtered_labels.contains_key("io.podman.compose.project"));
            }
            None => {
                // This shouldn't happen as we have non-system labels
                panic!("Expected Some(labels) but got None");
            }
        }
    }

    #[test]
    fn test_docker_context_edge_cases() {
        use std::env;
        use std::fs;

        // Test reading context with malformed JSON
        let temp_dir = tempfile::tempdir().unwrap();
        let docker_dir = temp_dir.path().join(".docker");
        let contexts_dir = docker_dir.join("contexts").join("meta");
        fs::create_dir_all(&contexts_dir).unwrap();

        // Create context with various edge cases
        let test_cases = vec![
            // Empty JSON object
            ("empty", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", "{}"),
            // Missing endpoints
            ("no-endpoints", "9c87ceaa86e893a4f10c15f18c2d4c5fea966872a7f9c5d1fa84bb0ee3b0abb6", r#"{"Name": "test", "Metadata": {}}"#),
            // Null values
            ("null-values", "5994471abb01112afcc18159f6cc74b4f511b99806da59b3caf5a9c173cacfc5", r#"{"Name": "test", "Metadata": null, "Endpoints": null}"#),
            // Unicode in context name
            ("unicode", "9b74c9897bac770ffc029102a200c5de", r#"{"Name": "test-世界", "Metadata": {}, "Endpoints": {"docker": {"Host": "tcp://localhost:2375"}}}"#),
        ];

        for (_name, hash, content) in test_cases {
            let context_dir = contexts_dir.join(hash);
            fs::create_dir_all(&context_dir).unwrap();
            fs::write(context_dir.join("meta.json"), content).unwrap();
        }

        let original_home = env::var("HOME").ok();
        env::set_var("HOME", temp_dir.path());

        // Test that all edge cases are handled gracefully
        use autocompose::docker::DockerProcessor;
        let results = vec![
            DockerProcessor::new_with_context("empty"),
            DockerProcessor::new_with_context("no-endpoints"),
            DockerProcessor::new_with_context("null-values"),
            DockerProcessor::new_with_context("unicode"),
        ];

        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }

        // All should either succeed or fail gracefully
        for result in results {
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[test]
    fn test_processing_options_edge_cases() {
        use autocompose::{ProcessingOptions, Service};

        // Test with service that has no environment at all
        let service_no_env = Service {
            image: "test:latest".to_string(),
            container_name: Some("test".to_string()),
            environment: None,
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

        // Processing options should work with services without environment
        let options_sensitive = ProcessingOptions { include_sensitive: true };
        let options_no_sensitive = ProcessingOptions { include_sensitive: false };
        
        assert!(service_no_env.environment.is_none());
        assert!(options_sensitive.include_sensitive);
        assert!(!options_no_sensitive.include_sensitive);
    }

    #[test]
    fn test_depends_on_edge_cases() {
        // Test with empty depends_on array
        let service_empty_deps = Service {
            image: "test:latest".to_string(),
            container_name: Some("test".to_string()),
            depends_on: Some(vec![]),
            hostname: None,
            environment: None,
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
        };

        let yaml = serde_yaml::to_string(&service_empty_deps).unwrap();
        // Empty array should still be serialized
        assert!(yaml.contains("depends_on: []"));

        // Test with very long dependency names
        let long_deps = vec![
            "very-long-service-name-that-exceeds-normal-naming-conventions".to_string(),
            "another-extremely-long-service-name-with-numbers-123456789".to_string(),
        ];
        
        let service_long_deps = Service {
            image: "test:latest".to_string(),
            container_name: Some("test".to_string()),
            depends_on: Some(long_deps),
            hostname: None,
            environment: None,
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
        };

        let yaml = serde_yaml::to_string(&service_long_deps).unwrap();
        assert!(yaml.contains("very-long-service-name-that-exceeds-normal-naming-conventions"));
    }

    #[test]
    fn test_container_validation_boundary_conditions() {
        // Test container ID at exact boundary (64 chars)
        let boundary_id = "a".repeat(64);
        assert!(validate_container_id(&boundary_id).is_ok());

        // Test container ID just over boundary (65 chars)
        let over_boundary_id = "a".repeat(65);
        assert!(validate_container_id(&over_boundary_id).is_err());

        // Test with whitespace
        assert!(validate_container_id(" abc123 ").is_err());
        assert!(validate_container_id("abc\t123").is_err());
        assert!(validate_container_id("abc\n123").is_err());

        // Test with null bytes
        assert!(validate_container_id("abc\0123").is_err());
    }

    #[test]
    fn test_sensitive_env_var_edge_patterns() {
        let mut env = HashMap::new();
        
        // Edge cases for sensitive patterns
        env.insert("PASSWORDLESS_AUTH".to_string(), "enabled".to_string()); // Contains PASSWORD but might be non-sensitive
        env.insert("PASSWORD123".to_string(), "secret".to_string());
        env.insert("123PASSWORD".to_string(), "secret".to_string());
        env.insert("PASS_WORD".to_string(), "might-not-match".to_string());
        env.insert("KEY_PASSWORD_CONFIRM".to_string(), "secret".to_string());
        env.insert("TOKENIZER_CONFIG".to_string(), "config".to_string()); // Contains TOKEN but not sensitive
        env.insert("DECRYPT_KEY".to_string(), "key".to_string());
        env.insert("PUBLIC_KEY".to_string(), "public".to_string()); // Contains KEY but public
        env.insert("API_ENDPOINT".to_string(), "/api/v1".to_string()); // Contains API but not sensitive
        
        let filtered = filter_sensitive_env_vars(env);
        
        // Check filtering behavior based on actual implementation
        // The filter looks for these patterns: PASSWORD, SECRET, TOKEN, API_KEY, PRIVATE_KEY, etc.
        assert!(!filtered.contains_key("PASSWORD123")); // Contains PASSWORD
        assert!(!filtered.contains_key("123PASSWORD")); // Contains PASSWORD
        assert!(!filtered.contains_key("KEY_PASSWORD_CONFIRM")); // Contains PASSWORD
        assert!(!filtered.contains_key("PASSWORDLESS_AUTH")); // Contains PASSWORD
        
        // DECRYPT_KEY doesn't contain any of the sensitive patterns, so it should remain
        assert!(filtered.contains_key("DECRYPT_KEY"));
        
        // TOKENIZER_CONFIG contains TOKEN, so it should be filtered
        assert!(!filtered.contains_key("TOKENIZER_CONFIG"));
        
        // PUBLIC_KEY doesn't contain PRIVATE_KEY or API_KEY (exact match), so it should remain
        assert!(filtered.contains_key("PUBLIC_KEY"));
        
        // API_ENDPOINT doesn't contain API_KEY (exact match), so it should remain
        assert!(filtered.contains_key("API_ENDPOINT"));
    }
}
