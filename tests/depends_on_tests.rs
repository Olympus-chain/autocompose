/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

#[cfg(test)]
mod depends_on_tests {
    use autocompose::{ComposeFile, Service};
    use std::collections::HashMap;

    fn create_service_with_dependencies(deps: Vec<String>) -> Service {
        Service {
            image: "test:latest".to_string(),
            container_name: Some("test-service".to_string()),
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
            depends_on: if deps.is_empty() { None } else { Some(deps) },
        }
    }

    #[test]
    fn test_service_without_dependencies() {
        let service = create_service_with_dependencies(vec![]);
        assert!(service.depends_on.is_none());
        
        // Test serialization
        let yaml = serde_yaml::to_string(&service).unwrap();
        assert!(!yaml.contains("depends_on"));
    }

    #[test]
    fn test_service_with_single_dependency() {
        let service = create_service_with_dependencies(vec!["database".to_string()]);
        assert!(service.depends_on.is_some());
        assert_eq!(service.depends_on.as_ref().unwrap().len(), 1);
        assert_eq!(service.depends_on.as_ref().unwrap()[0], "database");
        
        // Test serialization
        let yaml = serde_yaml::to_string(&service).unwrap();
        assert!(yaml.contains("depends_on"));
        assert!(yaml.contains("- database"));
    }

    #[test]
    fn test_service_with_multiple_dependencies() {
        let deps = vec![
            "database".to_string(),
            "cache".to_string(),
            "message-queue".to_string(),
        ];
        let service = create_service_with_dependencies(deps.clone());
        
        assert!(service.depends_on.is_some());
        let service_deps = service.depends_on.as_ref().unwrap();
        assert_eq!(service_deps.len(), 3);
        assert!(service_deps.contains(&"database".to_string()));
        assert!(service_deps.contains(&"cache".to_string()));
        assert!(service_deps.contains(&"message-queue".to_string()));
        
        // Test serialization
        let yaml = serde_yaml::to_string(&service).unwrap();
        assert!(yaml.contains("depends_on"));
        assert!(yaml.contains("- database"));
        assert!(yaml.contains("- cache"));
        assert!(yaml.contains("- message-queue"));
    }

    #[test]
    fn test_compose_file_with_dependencies() {
        let mut services = HashMap::new();
        
        // Database service (no dependencies)
        services.insert(
            "database".to_string(),
            create_service_with_dependencies(vec![]),
        );
        
        // Cache service (no dependencies)
        services.insert(
            "cache".to_string(),
            create_service_with_dependencies(vec![]),
        );
        
        // API service (depends on database and cache)
        services.insert(
            "api".to_string(),
            create_service_with_dependencies(vec![
                "database".to_string(),
                "cache".to_string(),
            ]),
        );
        
        // Frontend service (depends on API)
        services.insert(
            "frontend".to_string(),
            create_service_with_dependencies(vec!["api".to_string()]),
        );
        
        let compose = ComposeFile {
            version: "3.9".to_string(),
            services,
            networks: None,
            volumes: None,
        };
        
        // Test serialization
        let yaml = serde_yaml::to_string(&compose).unwrap();
        
        // Verify structure
        assert!(yaml.contains("version: '3.9'"));
        assert!(yaml.contains("services:"));
        
        // Verify that services with dependencies have the depends_on field
        assert!(yaml.contains("depends_on"));
        
        // Find API service section and verify dependencies
        let yaml_lines: Vec<&str> = yaml.lines().collect();
        let mut in_api_section = false;
        let mut found_api_depends = false;
        let mut in_frontend_section = false;
        let mut found_frontend_depends = false;
        
        for line in yaml_lines {
            if line.trim() == "api:" {
                in_api_section = true;
                in_frontend_section = false;
            } else if line.trim() == "frontend:" {
                in_frontend_section = true;
                in_api_section = false;
            } else if line.starts_with("  ") && !line.starts_with("    ") {
                // New service starts
                in_api_section = false;
                in_frontend_section = false;
            }
            
            if in_api_section && line.contains("depends_on") {
                found_api_depends = true;
            }
            if in_frontend_section && line.contains("depends_on") {
                found_frontend_depends = true;
            }
        }
        
        assert!(found_api_depends, "API service should have depends_on");
        assert!(found_frontend_depends, "Frontend service should have depends_on");
        assert!(yaml.contains("- database"));
        assert!(yaml.contains("- cache"));
        assert!(yaml.contains("- api"));
    }

    #[test]
    fn test_circular_dependency_representation() {
        // Note: This test only verifies that circular dependencies can be represented
        // Actual circular dependency detection would be handled at runtime
        let mut services = HashMap::new();
        
        // Service A depends on B
        services.insert(
            "service-a".to_string(),
            create_service_with_dependencies(vec!["service-b".to_string()]),
        );
        
        // Service B depends on C
        services.insert(
            "service-b".to_string(),
            create_service_with_dependencies(vec!["service-c".to_string()]),
        );
        
        // Service C depends on A (circular)
        services.insert(
            "service-c".to_string(),
            create_service_with_dependencies(vec!["service-a".to_string()]),
        );
        
        let compose = ComposeFile {
            version: "3.9".to_string(),
            services,
            networks: None,
            volumes: None,
        };
        
        // Should serialize without errors (validation is runtime concern)
        let yaml = serde_yaml::to_string(&compose).unwrap();
        assert!(yaml.contains("depends_on"));
    }

    #[test]
    fn test_dependency_to_nonexistent_service() {
        // Test that we can represent dependencies to non-existent services
        // (validation would happen at runtime)
        let service = create_service_with_dependencies(vec![
            "nonexistent-service".to_string(),
            "another-missing-service".to_string(),
        ]);
        
        assert!(service.depends_on.is_some());
        let deps = service.depends_on.as_ref().unwrap();
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&"nonexistent-service".to_string()));
        assert!(deps.contains(&"another-missing-service".to_string()));
    }

    #[test]
    fn test_deserialization_with_depends_on() {
        let yaml = r#"
image: test:latest
container_name: test-service
depends_on:
  - database
  - cache
"#;
        
        let service: Service = serde_yaml::from_str(yaml).unwrap();
        assert!(service.depends_on.is_some());
        let deps = service.depends_on.unwrap();
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&"database".to_string()));
        assert!(deps.contains(&"cache".to_string()));
    }

    #[test]
    fn test_complex_dependency_chain() {
        let mut services = HashMap::new();
        
        // Create a complex dependency chain:
        // frontend -> api -> [database, cache, auth]
        // auth -> database
        // worker -> [database, cache]
        
        services.insert(
            "database".to_string(),
            create_service_with_dependencies(vec![]),
        );
        
        services.insert(
            "cache".to_string(),
            create_service_with_dependencies(vec![]),
        );
        
        services.insert(
            "auth".to_string(),
            create_service_with_dependencies(vec!["database".to_string()]),
        );
        
        services.insert(
            "api".to_string(),
            create_service_with_dependencies(vec![
                "database".to_string(),
                "cache".to_string(),
                "auth".to_string(),
            ]),
        );
        
        services.insert(
            "frontend".to_string(),
            create_service_with_dependencies(vec!["api".to_string()]),
        );
        
        services.insert(
            "worker".to_string(),
            create_service_with_dependencies(vec![
                "database".to_string(),
                "cache".to_string(),
            ]),
        );
        
        let compose = ComposeFile {
            version: "3.9".to_string(),
            services,
            networks: None,
            volumes: None,
        };
        
        // Serialize and verify
        let yaml = serde_yaml::to_string(&compose).unwrap();
        assert!(yaml.contains("depends_on"));
        
        // Deserialize back and verify
        let deserialized: ComposeFile = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized.services.len(), 6);
        
        // Verify API dependencies
        let api = deserialized.services.get("api").unwrap();
        assert!(api.depends_on.is_some());
        assert_eq!(api.depends_on.as_ref().unwrap().len(), 3);
    }
}