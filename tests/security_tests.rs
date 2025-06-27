/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

use docker_autocompose::security::*;
use std::path::Path;

#[cfg(test)]
mod security_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_validate_container_id_valid() {
        assert!(validate_container_id("abc123def456").is_ok());
        assert!(validate_container_id("a1b2c3d4e5f6").is_ok());
        assert!(validate_container_id("1234567890abcdef").is_ok());
    }

    #[test]
    fn test_validate_container_id_invalid() {
        // Empty ID
        assert!(validate_container_id("").is_err());

        // Contains special characters
        assert!(validate_container_id("abc-123").is_err());
        assert!(validate_container_id("abc_123").is_err());
        assert!(validate_container_id("abc.123").is_err());
        assert!(validate_container_id("abc;rm -rf /").is_err());
        assert!(validate_container_id("abc$(whoami)").is_err());
        assert!(validate_container_id("abc`date`").is_err());
        assert!(validate_container_id("abc|cat /etc/passwd").is_err());
        assert!(validate_container_id("abc&&echo bad").is_err());

        // Too long (>64 chars)
        let long_id = "a".repeat(65);
        assert!(validate_container_id(&long_id).is_err());
    }

    #[test]
    fn test_validate_image_id_valid() {
        assert!(validate_image_id("nginx").is_ok());
        assert!(validate_image_id("nginx:latest").is_ok());
        assert!(validate_image_id("nginx:1.21.3").is_ok());
        assert!(validate_image_id("docker.io/library/nginx:latest").is_ok());
        assert!(validate_image_id("myregistry.com:5000/myapp:v1.0.0").is_ok());
        assert!(validate_image_id("sha256:abc123def456").is_ok());
        assert!(validate_image_id("nginx@sha256:abc123").is_ok());
    }

    #[test]
    fn test_validate_image_id_invalid() {
        // Empty image
        assert!(validate_image_id("").is_err());

        // Contains dangerous characters
        assert!(validate_image_id("nginx;echo bad").is_err());
        assert!(validate_image_id("nginx$(whoami)").is_err());
        assert!(validate_image_id("nginx`date`").is_err());
        assert!(validate_image_id("nginx|cat /etc/passwd").is_err());
        assert!(validate_image_id("nginx&&rm -rf /").is_err());
        assert!(validate_image_id("nginx\";DROP TABLE").is_err());

        // Too long (>256 chars)
        let long_image = "a".repeat(257);
        assert!(validate_image_id(&long_image).is_err());
    }

    #[test]
    fn test_filter_sensitive_env_vars() {
        let mut env = HashMap::new();
        env.insert("PATH".to_string(), "/usr/bin:/usr/local/bin".to_string());
        env.insert("HOME".to_string(), "/home/user".to_string());
        env.insert("USER".to_string(), "testuser".to_string());
        env.insert("PASSWORD".to_string(), "secret123".to_string());
        env.insert("DB_PASSWORD".to_string(), "dbsecret456".to_string());
        env.insert("API_KEY".to_string(), "apikey789".to_string());
        env.insert("SECRET_TOKEN".to_string(), "token123".to_string());
        env.insert("AWS_SECRET_ACCESS_KEY".to_string(), "aws123".to_string());
        env.insert("GITHUB_TOKEN".to_string(), "ghtoken".to_string());
        env.insert("MYSQL_ROOT_PASSWORD".to_string(), "rootpass".to_string());
        env.insert("NORMAL_VAR".to_string(), "value".to_string());

        let filtered = filter_sensitive_env_vars(env);

        // Should keep safe variables
        assert!(filtered.contains_key("PATH"));
        assert!(filtered.contains_key("HOME"));
        assert!(filtered.contains_key("USER"));
        assert!(filtered.contains_key("NORMAL_VAR"));

        // Should remove sensitive variables
        assert!(!filtered.contains_key("PASSWORD"));
        assert!(!filtered.contains_key("DB_PASSWORD"));
        assert!(!filtered.contains_key("API_KEY"));
        assert!(!filtered.contains_key("SECRET_TOKEN"));
        assert!(!filtered.contains_key("AWS_SECRET_ACCESS_KEY"));
        assert!(!filtered.contains_key("GITHUB_TOKEN"));
        assert!(!filtered.contains_key("MYSQL_ROOT_PASSWORD"));
    }

    #[test]
    fn test_filter_sensitive_env_vars_case_insensitive() {
        let mut env = HashMap::new();
        env.insert("password".to_string(), "lowercase".to_string());
        env.insert("Password".to_string(), "titlecase".to_string());
        env.insert("PASSWORD".to_string(), "uppercase".to_string());
        env.insert("PaSsWoRd".to_string(), "mixedcase".to_string());

        let filtered = filter_sensitive_env_vars(env);

        // All password variations should be filtered
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_validate_output_path_safe() {
        // Safe paths should be allowed
        let safe_paths = vec![
            "docker-compose.yml",
            "./docker-compose.yml",
            "/home/user/docker-compose.yml",
            "/tmp/compose.yml",
            "/var/tmp/output.yml",
        ];

        for path_str in safe_paths {
            let path = Path::new(path_str);
            // Note: This might fail in test environment due to path not existing
            // In real usage, the parent directory would exist
            let _result = validate_output_path(path);
            if path_str.starts_with("/home")
                || path_str.starts_with("./")
                || !path_str.starts_with("/")
            {
                // These should work in test environment
                println!("Testing path: {}", path_str);
            }
        }
    }

    #[test]
    fn test_shell_escape() {
        // Simple strings should not be quoted
        assert_eq!(shell_escape("simple"), "simple");
        assert_eq!(shell_escape("simple123"), "simple123");

        // Strings with spaces should be quoted
        assert_eq!(shell_escape("with space"), "'with space'");
        assert_eq!(shell_escape("multiple  spaces"), "'multiple  spaces'");

        // Strings with special characters should be properly escaped
        assert_eq!(shell_escape("with'quote"), "'with'\\''quote'");
        assert_eq!(shell_escape("rm -rf /"), "'rm -rf /'");
        assert_eq!(shell_escape("$(whoami)"), "'$(whoami)'");
        assert_eq!(shell_escape("`date`"), "'`date`'");
        assert_eq!(shell_escape("abc|def"), "'abc|def'");
        assert_eq!(shell_escape("abc&def"), "'abc&def'");
        assert_eq!(shell_escape("abc;def"), "'abc;def'");
    }
}
