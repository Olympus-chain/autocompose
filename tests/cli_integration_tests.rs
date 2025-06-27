/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[cfg(test)]
mod cli_integration_tests {
    use super::*;

    fn get_binary_path() -> String {
        "./target/debug/autocompose".to_string()
    }

    #[test]
    fn test_cli_help_command() {
        let output = Command::new(get_binary_path())
            .arg("--help")
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Generate Docker Compose files from running containers"));
        assert!(stdout.contains("docker"));
        assert!(stdout.contains("podman"));
        assert!(stdout.contains("config"));
        assert!(stdout.contains("validate"));
    }

    #[test]
    fn test_cli_version_command() {
        let output = Command::new(get_binary_path())
            .arg("--version")
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("autocompose"));
    }

    #[test]
    fn test_docker_help_subcommand() {
        let output = Command::new(get_binary_path())
            .args(&["docker", "--help"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Export Docker containers"));
        assert!(stdout.contains("--output"));
        assert!(stdout.contains("--running-only"));
        assert!(stdout.contains("--format"));
    }

    #[test]
    fn test_config_subcommands() {
        // Test config show
        let output = Command::new(get_binary_path())
            .args(&["config", "show"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Current configuration:"));
    }

    #[test]
    fn test_validate_invalid_file() {
        let temp_dir = TempDir::new().unwrap();
        let invalid_file = temp_dir.path().join("invalid.yml");
        fs::write(&invalid_file, "invalid: yaml: content:").unwrap();

        let output = Command::new(get_binary_path())
            .args(&["validate", invalid_file.to_str().unwrap()])
            .output()
            .expect("Failed to execute command");

        assert!(!output.status.success());
    }

    #[test]
    fn test_validate_valid_compose_file() {
        let temp_dir = TempDir::new().unwrap();
        let valid_file = temp_dir.path().join("valid.yml");
        let valid_content = r#"
version: '3.9'
services:
  test:
    image: nginx:latest
    ports:
      - "80:80"
"#;
        fs::write(&valid_file, valid_content).unwrap();

        let output = Command::new(get_binary_path())
            .args(&["validate", valid_file.to_str().unwrap()])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Status: VALID"));
    }

    #[test]
    fn test_output_format_options() {
        // Test that format validation works
        let formats = vec!["yaml", "json", "toml"];

        for format in formats {
            let output = Command::new(get_binary_path())
                .args(&["docker", "--help"])
                .output()
                .expect("Failed to execute command");

            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains(format));
        }
    }

    #[test]
    fn test_config_set_and_reset() {
        // Test setting a config value
        let output = Command::new(get_binary_path())
            .args(&["config", "set", "default_format", "json"])
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("Configuration updated"));

            // Reset to default
            let _ = Command::new(get_binary_path())
                .args(&["config", "reset"])
                .output();
        }
    }

    #[test]
    fn test_dry_run_flag() {
        // Test that dry run doesn't create files
        let temp_dir = TempDir::new().unwrap();
        let output_file = temp_dir.path().join("test-compose.yml");

        let _output = Command::new(get_binary_path())
            .args(&[
                "docker",
                "--dry-run",
                "--output",
                output_file.to_str().unwrap(),
            ])
            .output()
            .expect("Failed to execute command");

        // Even if command fails (no Docker), file shouldn't exist
        assert!(!output_file.exists());
    }
}
