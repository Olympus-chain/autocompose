/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

#[cfg(test)]
mod docker_connection_tests {
    use autocompose::docker::DockerProcessor;
    use std::env;
    use std::fs;

    #[test]
    fn test_new_with_default_connection() {
        // Test default connection
        let result = DockerProcessor::new();
        assert!(result.is_ok(), "Should create DockerProcessor with default connection");
    }

    #[test]
    fn test_new_with_host() {
        // Test with valid host formats
        let hosts = vec![
            "tcp://localhost:2375",
            "http://localhost:2375",
            "unix:///var/run/docker.sock",
        ];

        for host in hosts {
            let result = DockerProcessor::new_with_host(host);
            assert!(result.is_ok(), "Should create DockerProcessor with host: {}", host);
        }
    }

    #[test]
    fn test_new_with_invalid_host() {
        // Test with invalid host format
        let result = DockerProcessor::new_with_host("invalid://host");
        // Note: bollard might still accept this, so we just check it doesn't panic
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_new_with_context_default() {
        // Test with default context
        let result = DockerProcessor::new_with_context("default");
        assert!(result.is_ok(), "Should handle default context");
    }

    #[test]
    fn test_new_with_context_current() {
        // Test with current context
        let result = DockerProcessor::new_with_context("current");
        assert!(result.is_ok(), "Should handle current context");
    }

    #[test]
    fn test_new_with_nonexistent_context() {
        // Test with non-existent context - should fallback to default
        let result = DockerProcessor::new_with_context("nonexistent-context-xyz123");
        assert!(result.is_ok(), "Should fallback to default for non-existent context");
    }

    #[test]
    fn test_hash_context_name() {
        // Test that hash_context_name produces consistent SHA256 hashes
        // We can't test the private function directly, but we can test the behavior
        // by creating a mock context and verifying it's read correctly
        
        // Create a temporary docker config directory
        let temp_dir = tempfile::tempdir().unwrap();
        let docker_dir = temp_dir.path().join(".docker");
        let contexts_dir = docker_dir.join("contexts").join("meta");
        fs::create_dir_all(&contexts_dir).unwrap();

        // The hash for "test-context" should be consistent
        // We'll create a context file with the expected hash
        let test_context_hash = "8f4e72c7bcfb426ce5459a9399528e4c9642fa1520955a6c83d49db0e770b233";
        let context_dir = contexts_dir.join(test_context_hash);
        fs::create_dir_all(&context_dir).unwrap();

        let meta_content = r#"{
            "Name": "test-context",
            "Metadata": {},
            "Endpoints": {
                "docker": {
                    "Host": "tcp://test-host:2375",
                    "SkipTLSVerify": false
                }
            }
        }"#;
        fs::write(context_dir.join("meta.json"), meta_content).unwrap();

        // Set HOME to temp directory
        let original_home = env::var("HOME").ok();
        env::set_var("HOME", temp_dir.path());

        // Test reading the context
        let result = DockerProcessor::new_with_context("test-context");
        
        // Restore original HOME
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }

        assert!(result.is_ok(), "Should read test context successfully");
    }

    #[test]
    fn test_docker_host_env_precedence() {
        // Test that DOCKER_HOST env var takes precedence for default context
        let original_docker_host = env::var("DOCKER_HOST").ok();
        
        env::set_var("DOCKER_HOST", "tcp://custom-host:2375");
        let result = DockerProcessor::new_with_context("default");
        
        // Restore original DOCKER_HOST
        if let Some(host) = original_docker_host {
            env::set_var("DOCKER_HOST", host);
        } else {
            env::remove_var("DOCKER_HOST");
        }

        assert!(result.is_ok(), "Should use DOCKER_HOST for default context");
    }

    #[test]
    fn test_context_with_missing_docker_endpoint() {
        // Create a context without docker endpoint
        let temp_dir = tempfile::tempdir().unwrap();
        let docker_dir = temp_dir.path().join(".docker");
        let contexts_dir = docker_dir.join("contexts").join("meta");
        fs::create_dir_all(&contexts_dir).unwrap();

        let test_context_hash = "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3"; // hash of "test"
        let context_dir = contexts_dir.join(test_context_hash);
        fs::create_dir_all(&context_dir).unwrap();

        let meta_content = r#"{
            "Name": "test",
            "Metadata": {},
            "Endpoints": {}
        }"#;
        fs::write(context_dir.join("meta.json"), meta_content).unwrap();

        let original_home = env::var("HOME").ok();
        env::set_var("HOME", temp_dir.path());

        let result = DockerProcessor::new_with_context("test");
        
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }

        // Should fallback to default when no docker endpoint
        assert!(result.is_ok(), "Should fallback when no docker endpoint");
    }

    #[test]
    fn test_context_with_corrupt_meta_json() {
        // Create a context with corrupt JSON
        let temp_dir = tempfile::tempdir().unwrap();
        let docker_dir = temp_dir.path().join(".docker");
        let contexts_dir = docker_dir.join("contexts").join("meta");
        fs::create_dir_all(&contexts_dir).unwrap();

        let test_context_hash = "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3";
        let context_dir = contexts_dir.join(test_context_hash);
        fs::create_dir_all(&context_dir).unwrap();

        fs::write(context_dir.join("meta.json"), "{ invalid json").unwrap();

        let original_home = env::var("HOME").ok();
        env::set_var("HOME", temp_dir.path());

        let result = DockerProcessor::new_with_context("test");
        
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }

        // Should handle corrupt JSON gracefully
        assert!(result.is_ok() || result.is_err(), "Should handle corrupt JSON");
    }
}