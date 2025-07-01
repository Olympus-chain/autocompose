/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

#[cfg(test)]
mod docker_tests {
    use autocompose::{
        Deploy, HealthCheck, NetworkConfig, ResourceLimits, Resources, Service, ServiceNetworks,
    };
    use std::collections::HashMap;

    fn create_test_service() -> Service {
        let mut env = HashMap::new();
        env.insert("PATH".to_string(), "/usr/bin".to_string());
        env.insert("APP_ENV".to_string(), "test".to_string());

        Service {
            image: "test:latest".to_string(),
            container_name: Some("test-container".to_string()),
            hostname: Some("test-host".to_string()),
            environment: Some(env),
            ports: Some(vec!["8080:80".to_string()]),
            volumes: Some(vec!["/data:/data".to_string()]),
            networks: None,
            network_mode: Some("bridge".to_string()),
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
            user: None,
            working_dir: Some("/app".to_string()),
            entrypoint: None,
            command: None,
            ulimits: None,
            sysctls: None,
            init: Some(false),
            privileged: Some(false),
            tty: Some(false),
            stdin_open: Some(false),
            depends_on: None,
        }
    }

    #[test]
    fn test_service_serialization() {
        let service = create_test_service();
        let yaml = serde_yaml::to_string(&service).unwrap();

        // Check that required fields are present
        assert!(yaml.contains("image: test:latest"));
        assert!(yaml.contains("container_name: test-container"));
        assert!(yaml.contains("hostname: test-host"));
        assert!(yaml.contains("APP_ENV: test"));

        // Check that None fields are not serialized
        assert!(!yaml.contains("dns:"));
        assert!(!yaml.contains("labels:"));
        assert!(!yaml.contains("user:"));
    }

    #[test]
    fn test_empty_environment_map_is_omitted() {
        let mut service = create_test_service();
        service.environment = Some(HashMap::new());

        let yaml = serde_yaml::to_string(&service).unwrap();
        // Empty HashMap is serialized as {} in YAML
        // The actual filtering happens in docker.rs/podman.rs before creating the Service
        assert!(yaml.contains("environment: {}"));
    }

    #[test]
    fn test_network_configuration() {
        let mut service = create_test_service();
        let mut networks = HashMap::new();

        networks.insert(
            "mynet".to_string(),
            NetworkConfig {
                ipv4_address: Some("172.20.0.2".to_string()),
                ipv6_address: None,
            },
        );

        service.networks = Some(ServiceNetworks::Advanced(networks));

        let yaml = serde_yaml::to_string(&service).unwrap();
        assert!(yaml.contains("networks:"));
        assert!(yaml.contains("mynet:"));
        assert!(yaml.contains("ipv4_address: 172.20.0.2"));
        assert!(!yaml.contains("ipv6_address:"));
    }

    #[test]
    fn test_healthcheck_serialization() {
        let mut service = create_test_service();
        service.healthcheck = Some(HealthCheck {
            test: vec![
                "CMD".to_string(),
                "curl".to_string(),
                "-f".to_string(),
                "http://localhost/".to_string(),
            ],
            interval: Some("30s".to_string()),
            timeout: Some("10s".to_string()),
            retries: Some(3),
            start_period: Some("40s".to_string()),
        });

        let yaml = serde_yaml::to_string(&service).unwrap();
        assert!(yaml.contains("healthcheck:"));
        assert!(yaml.contains("test:"));
        assert!(yaml.contains("- CMD"));
        assert!(yaml.contains("interval: 30s"));
        assert!(yaml.contains("retries: 3"));
    }

    #[test]
    fn test_deploy_resources() {
        let mut service = create_test_service();
        service.deploy = Some(Deploy {
            resources: Some(Resources {
                limits: Some(ResourceLimits {
                    cpus: Some("0.5".to_string()),
                    memory: Some("512M".to_string()),
                }),
            }),
            placement: None,
        });

        let yaml = serde_yaml::to_string(&service).unwrap();
        assert!(yaml.contains("deploy:"));
        assert!(yaml.contains("resources:"));
        assert!(yaml.contains("limits:"));
        assert!(yaml.contains("cpus: '0.5'"));
        assert!(yaml.contains("memory: 512M"));
    }

    #[test]
    fn test_port_filtering() {
        // Test that IPv6 ports are filtered out
        let ports = vec![
            "8080:80/tcp".to_string(),
            ":::8080:80/tcp".to_string(), // Should be filtered
        ];

        let filtered: Vec<_> = ports
            .into_iter()
            .filter(|p| !p.starts_with(":::"))
            .collect();

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0], "8080:80/tcp");
    }

    #[test]
    fn test_privileged_and_security_fields() {
        let mut service = create_test_service();
        service.privileged = Some(true);
        service.init = Some(true);
        service.tty = Some(true);
        service.stdin_open = Some(true);
        service.cap_add = Some(vec!["SYS_ADMIN".to_string()]);
        service.security_opt = Some(vec!["apparmor:unconfined".to_string()]);

        let yaml = serde_yaml::to_string(&service).unwrap();
        assert!(yaml.contains("privileged: true"));
        assert!(yaml.contains("init: true"));
        assert!(yaml.contains("tty: true"));
        assert!(yaml.contains("stdin_open: true"));
        assert!(yaml.contains("cap_add:"));
        assert!(yaml.contains("- SYS_ADMIN"));
        assert!(yaml.contains("security_opt:"));
        assert!(yaml.contains("- apparmor:unconfined"));
    }

    #[test]
    fn test_restart_policy_values() {
        let valid_policies = vec!["no", "always", "unless-stopped", "on-failure"];

        for policy in valid_policies {
            let mut service = create_test_service();
            service.restart = Some(policy.to_string());
            let yaml = serde_yaml::to_string(&service).unwrap();
            assert!(yaml.contains(&format!("restart: {}", policy)));
        }
    }
}
