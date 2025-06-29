/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

use crate::constants::{MAX_CONTAINER_ID_LENGTH, MAX_IMAGE_ID_LENGTH};
use crate::{AutoComposeError, Result};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Validates that a container ID is safe to use in shell commands
pub fn validate_container_id(id: &str) -> Result<&str> {
    //  Alphanumeric characters
    if id.is_empty() || id.len() > MAX_CONTAINER_ID_LENGTH {
        return Err(AutoComposeError::Validation(
            "Invalid container ID length".to_string(),
        ));
    }

    if !id.chars().all(|c| c.is_alphanumeric()) {
        return Err(AutoComposeError::Validation(
            "Container ID contains invalid characters".to_string(),
        ));
    }

    Ok(id)
}

/// Validates that an image ID/name is safe to use in shell commands
pub fn validate_image_id(image: &str) -> Result<&str> {
    if image.is_empty() || image.len() > MAX_IMAGE_ID_LENGTH {
        return Err(AutoComposeError::Validation(
            "Invalid image ID length".to_string(),
        ));
    }

    // Allow alphanumeric, dots, slashes, colons, hyphens, and underscores for image names
    if !image.chars().all(|c| {
        c.is_alphanumeric() || c == '.' || c == '/' || c == ':' || c == '-' || c == '_' || c == '@'
    }) {
        return Err(AutoComposeError::Validation(
            "Image ID contains invalid characters".to_string(),
        ));
    }

    Ok(image)
}

/// Validates output file path to prevent directory traversal
pub fn validate_output_path(path: &Path) -> Result<PathBuf> {
    // If path has no parent, use current directory
    let parent = path.parent().unwrap_or(Path::new("."));
    let file_name = path
        .file_name()
        .ok_or_else(|| AutoComposeError::Validation("Invalid file name".to_string()))?;

    // Canonicalize the parent directory
    let canonical_parent = parent
        .canonicalize()
        .or_else(|_| {
            // If parent doesn't exist, try current directory
            std::env::current_dir()
        })
        .map_err(|e| AutoComposeError::Validation(format!("Invalid output path: {}", e)))?;

    let canonical = canonical_parent.join(file_name);

    // Check if the path is trying to write to sensitive locations
    let sensitive_dirs = vec!["/etc", "/sys", "/proc", "/boot", "/dev"];
    let canonical_str = canonical.to_string_lossy();

    for sensitive in sensitive_dirs {
        if canonical_str.starts_with(sensitive) {
            return Err(AutoComposeError::Validation(format!(
                "Cannot write to sensitive directory: {}",
                sensitive
            )));
        }
    }

    Ok(canonical)
}

/// Filters out sensitive environment variables
pub fn filter_sensitive_env_vars(env_vars: HashMap<String, String>) -> HashMap<String, String> {
    lazy_static::lazy_static! {
        static ref SENSITIVE_PATTERNS: HashSet<&'static str> = {
            let mut set = HashSet::new();
            // Common sensitive variable patterns
            set.insert("PASSWORD");
            set.insert("SECRET");
            set.insert("TOKEN");
            set.insert("API_KEY");
            set.insert("PRIVATE_KEY");
            set.insert("ACCESS_KEY");
            set.insert("CREDENTIALS");
            set.insert("AUTH");
            set.insert("MYSQL_ROOT_PASSWORD");
            set.insert("POSTGRES_PASSWORD");
            set.insert("REDIS_PASSWORD");
            set.insert("MONGODB_PASSWORD");
            set.insert("RABBITMQ_PASSWORD");
            set.insert("AWS_SECRET");
            set.insert("GITHUB_TOKEN");
            set.insert("DOCKER_PASSWORD");
            set.insert("NPM_TOKEN");
            set
        };
    }

    env_vars
        .into_iter()
        .filter(|(key, _)| {
            let key_upper = key.to_uppercase();
            !SENSITIVE_PATTERNS
                .iter()
                .any(|pattern| key_upper.contains(pattern))
        })
        .collect()
}

/// Escapes a string for safe use in shell commands
pub fn shell_escape(s: &str) -> String {
    // Use shell-words crate for proper escaping
    shell_words::quote(s).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_validate_container_id() {
        assert!(validate_container_id("abc123def456").is_ok());
        assert!(validate_container_id("").is_err());
        assert!(validate_container_id("abc-123").is_err());
        assert!(validate_container_id("abc;rm -rf /").is_err());
        assert!(validate_container_id(&"a".repeat(65)).is_err());
    }

    #[test]
    fn test_validate_image_id() {
        assert!(validate_image_id("nginx:latest").is_ok());
        assert!(validate_image_id("docker.io/library/redis:alpine").is_ok());
        assert!(validate_image_id("sha256:abc123").is_ok());
        assert!(validate_image_id("nginx;echo bad").is_err());
        assert!(validate_image_id("").is_err());
    }

    #[test]
    fn test_filter_sensitive_env_vars() {
        let mut env = HashMap::new();
        env.insert("PATH".to_string(), "/usr/bin".to_string());
        env.insert("DB_PASSWORD".to_string(), "secret123".to_string());
        env.insert("API_KEY".to_string(), "key123".to_string());
        env.insert("HOME".to_string(), "/home/user".to_string());
        env.insert("SECRET_TOKEN".to_string(), "token123".to_string());

        let filtered = filter_sensitive_env_vars(env);

        assert_eq!(filtered.len(), 2);
        assert!(filtered.contains_key("PATH"));
        assert!(filtered.contains_key("HOME"));
        assert!(!filtered.contains_key("DB_PASSWORD"));
        assert!(!filtered.contains_key("API_KEY"));
        assert!(!filtered.contains_key("SECRET_TOKEN"));
    }

    #[test]
    fn test_shell_escape() {
        assert_eq!(shell_escape("simple"), "simple");
        assert_eq!(shell_escape("with space"), "'with space'");
        assert_eq!(shell_escape("with'quote"), "'with'\\''quote'");
        assert_eq!(shell_escape("rm -rf /"), "'rm -rf /'");
    }
}
