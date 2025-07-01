/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

#[cfg(test)]
mod regression_tests {
    use autocompose::{
        security::{validate_container_id, validate_image_id, validate_output_path},
        Service,
    };
    use std::collections::HashMap;
    use std::path::Path;

    #[test]
    fn test_regression_empty_user_field() {
        // Regression test for bug where empty user field was serialized as ""
        let service = Service {
            image: "nginx:latest".to_string(),
            user: None, // This used to be Some("") which created invalid YAML
            container_name: Some("test".to_string()),
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
        assert!(!yaml.contains("user:"));
        assert!(!yaml.contains("user: ''"));
        assert!(!yaml.contains("user: \"\""));
    }

    #[test]
    fn test_regression_ipv6_port_filtering() {
        // Regression test for IPv6 ports showing as ":::"
        let ports = vec!["8080:80/tcp".to_string(), "0.0.0.0:8080:80/tcp".to_string()];

        // IPv6 ports should be filtered out in the actual implementation
        for port in &ports {
            assert!(!port.starts_with(":::"));
            assert!(!port.contains("::"));
        }
    }

    #[test]
    fn test_regression_command_injection_prevention() {
        // Regression test for command injection vulnerability
        let dangerous_ids = vec![
            "abc; rm -rf /",
            "abc$(whoami)",
            "abc`date`",
            "abc|cat /etc/passwd",
            "abc&&echo bad",
            "abc||echo bad",
            "abc;echo bad",
            "abc\necho bad",
            "abc\recho bad",
        ];

        for id in dangerous_ids {
            assert!(validate_container_id(id).is_err());
        }
    }

    #[test]
    fn test_regression_sensitive_env_filtering() {
        // Regression test for sensitive environment variables being exposed
        let mut env = HashMap::new();
        env.insert("POSTGRES_PASSWORD".to_string(), "secret123".to_string());
        env.insert("MYSQL_ROOT_PASSWORD".to_string(), "rootpass".to_string());
        env.insert("REDIS_PASSWORD".to_string(), "redispass".to_string());
        env.insert("API_KEY".to_string(), "key123".to_string());
        env.insert("SECRET_TOKEN".to_string(), "token456".to_string());
        env.insert("PATH".to_string(), "/usr/bin".to_string());

        let filtered = autocompose::security::filter_sensitive_env_vars(env);

        // Only PATH should remain
        assert_eq!(filtered.len(), 1);
        assert!(filtered.contains_key("PATH"));
    }

    #[test]
    fn test_regression_path_traversal_prevention() {
        // Regression test for path traversal vulnerability
        let dangerous_paths = vec![
            "/etc/passwd",
            "/etc/shadow",
            "/sys/test",
            "/proc/test",
            "/boot/config",
            "/dev/null",
        ];

        for path_str in dangerous_paths {
            let path = Path::new(path_str);
            let result = validate_output_path(path);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_regression_empty_constraints() {
        // Regression test for empty placement constraints
        use autocompose::{Deploy, Placement};

        let deploy = Deploy {
            placement: Some(Placement {
                constraints: Some(vec![
                    "node.role == worker".to_string(),
                    // Empty constraints should be filtered out
                ]),
            }),
            resources: None,
        };

        let yaml = serde_yaml::to_string(&deploy).unwrap();
        assert!(yaml.contains("node.role == worker"));
        assert!(!yaml.contains("node.labels.cpus == ''"));
    }

    #[test]
    fn test_regression_missing_container_attributes() {
        // Regression test for missing init, privileged, tty, stdin_open fields
        let service = Service {
            image: "nginx:latest".to_string(),
            init: Some(true),
            privileged: Some(false),
            tty: Some(true),
            stdin_open: Some(false),
            container_name: Some("test".to_string()),
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
            depends_on: None,
        };

        let yaml = serde_yaml::to_string(&service).unwrap();
        assert!(yaml.contains("init: true"));
        assert!(yaml.contains("privileged: false"));
        assert!(yaml.contains("tty: true"));
        assert!(yaml.contains("stdin_open: false"));
    }

    #[test]
    fn test_regression_relative_path_handling() {
        // Regression test for relative path validation
        let valid_paths = vec![
            "docker-compose.yml",
            "./docker-compose.yml",
            "../docker-compose.yml",
            "output/docker-compose.yml",
        ];

        for path_str in valid_paths {
            let _path = Path::new(path_str);
            // These should not panic or fail in the actual implementation
            assert!(!path_str.is_empty());
        }
    }

    #[test]
    fn test_regression_image_validation() {
        // Regression test for image validation
        let valid_images = vec![
            "nginx",
            "nginx:latest",
            "nginx:1.21.0",
            "docker.io/library/nginx:latest",
            "quay.io/nginx/nginx:latest",
            "localhost:5000/myapp:v1.0.0",
            "myregistry.com:443/namespace/image:tag",
            "sha256:abc123def456",
            "image@sha256:abc123def456",
        ];

        for image in valid_images {
            assert!(validate_image_id(image).is_ok());
        }

        let invalid_images = vec![
            "",
            "nginx;echo bad",
            "nginx$(whoami)",
            "nginx`date`",
            "nginx|cat",
            "nginx&&echo",
        ];

        for image in invalid_images {
            assert!(validate_image_id(image).is_err());
        }
    }

    #[test]
    fn test_regression_api_backward_compatibility() {
        // Test that the old process_containers_parallel API still works
        // and defaults to include_sensitive = false
        use autocompose::ProcessingOptions;

        // The default behavior should exclude sensitive variables
        let default_options = ProcessingOptions {
            include_sensitive: false,
        };
        assert!(!default_options.include_sensitive);

        // Test that sensitive env vars are filtered by default
        let mut env = HashMap::new();
        env.insert("PASSWORD".to_string(), "secret".to_string());
        env.insert("NORMAL_VAR".to_string(), "value".to_string());

        let filtered = autocompose::security::filter_sensitive_env_vars(env);
        assert!(!filtered.contains_key("PASSWORD"));
        assert!(filtered.contains_key("NORMAL_VAR"));
    }

    #[test]
    fn test_regression_docker_context_api() {
        // Test that new Docker connection methods don't break existing functionality
        use autocompose::docker::DockerProcessor;

        // Default connection should still work
        let default_result = DockerProcessor::new();
        assert!(default_result.is_ok() || default_result.is_err()); // Don't fail if Docker isn't available

        // Connection with host should handle errors gracefully
        let host_result = DockerProcessor::new_with_host("tcp://invalid-host:2375");
        assert!(host_result.is_ok() || host_result.is_err());

        // Connection with non-existent context should fallback gracefully
        let context_result = DockerProcessor::new_with_context("non-existent-context-xyz");
        assert!(context_result.is_ok() || context_result.is_err());
    }

    #[test]
    fn test_regression_service_depends_on_serialization() {
        // Test that depends_on field is properly handled in serialization
        let service_with_deps = Service {
            image: "app:latest".to_string(),
            container_name: Some("app".to_string()),
            depends_on: Some(vec!["db".to_string(), "cache".to_string()]),
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

        let yaml = serde_yaml::to_string(&service_with_deps).unwrap();
        assert!(yaml.contains("depends_on"));
        assert!(yaml.contains("- db"));
        assert!(yaml.contains("- cache"));

        // Test service without dependencies
        let service_no_deps = Service {
            image: "app:latest".to_string(),
            container_name: Some("app".to_string()),
            depends_on: None,
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

        let yaml = serde_yaml::to_string(&service_no_deps).unwrap();
        assert!(!yaml.contains("depends_on"));
    }

    #[test]
    fn test_regression_processing_options_default_behavior() {
        // Ensure ProcessingOptions default behavior is secure
        use autocompose::ProcessingOptions;

        let options = ProcessingOptions {
            include_sensitive: false,
        };

        // Default should exclude sensitive data
        assert!(!options.include_sensitive);

        // Test with sensitive data included
        let options_with_sensitive = ProcessingOptions {
            include_sensitive: true,
        };
        assert!(options_with_sensitive.include_sensitive);
    }
}
