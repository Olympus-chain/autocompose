/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

use crate::{AutoComposeError, ComposeFile, Result, Service};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub suggestions: Vec<ValidationSuggestion>,
    pub summary: ValidationSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
    pub service: Option<String>,
    pub field: Option<String>,
    pub message: String,
    pub error_type: ErrorType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub service: Option<String>,
    pub field: Option<String>,
    pub message: String,
    pub warning_type: WarningType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationSuggestion {
    pub service: Option<String>,
    pub field: Option<String>,
    pub message: String,
    pub suggestion_type: SuggestionType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub total_services: usize,
    pub total_networks: usize,
    pub total_volumes: usize,
    pub compose_version: String,
    pub validation_time_ms: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorType {
    SyntaxError,
    InvalidVersion,
    MissingRequiredField,
    InvalidFieldValue,
    ConflictingConfiguration,
    UnsupportedFeature,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WarningType {
    SecurityRisk,
    PerformanceIssue,
    DeprecatedFeature,
    BestPracticeViolation,
    PortabilityIssue,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SuggestionType {
    SecurityImprovement,
    PerformanceOptimization,
    BestPractice,
    Modernization,
    Cleanup,
}

pub struct Validator {
    check_best_practices: bool,
    target_version: Option<String>,
}

impl Validator {
    pub fn new(check_best_practices: bool, target_version: Option<String>) -> Self {
        Self {
            check_best_practices,
            target_version,
        }
    }

    pub async fn validate_file<P: AsRef<Path>>(&self, file_path: P) -> Result<ValidationReport> {
        let start_time = std::time::Instant::now();

        let content = tokio::fs::read_to_string(&file_path).await?;
        let compose_file: ComposeFile = serde_yaml::from_str(&content)?;

        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut suggestions = Vec::new();

        self.validate_version(&compose_file, &mut errors, &mut warnings);
        self.validate_services(
            &compose_file.services,
            &mut errors,
            &mut warnings,
            &mut suggestions,
        );

        if self.check_best_practices {
            self.check_best_practices_compliance(&compose_file, &mut warnings, &mut suggestions);
        }

        let validation_time = start_time.elapsed().as_millis();

        let summary = ValidationSummary {
            total_services: compose_file.services.len(),
            total_networks: compose_file.networks.as_ref().map(|n| n.len()).unwrap_or(0),
            total_volumes: compose_file.volumes.as_ref().map(|v| v.len()).unwrap_or(0),
            compose_version: compose_file.version,
            validation_time_ms: validation_time,
        };

        Ok(ValidationReport {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            suggestions,
            summary,
        })
    }

    pub fn validate_compose_object(&self, compose_file: &ComposeFile) -> ValidationReport {
        let start_time = std::time::Instant::now();

        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut suggestions = Vec::new();

        self.validate_version(compose_file, &mut errors, &mut warnings);
        self.validate_services(
            &compose_file.services,
            &mut errors,
            &mut warnings,
            &mut suggestions,
        );

        if self.check_best_practices {
            self.check_best_practices_compliance(compose_file, &mut warnings, &mut suggestions);
        }

        let validation_time = start_time.elapsed().as_millis();

        let summary = ValidationSummary {
            total_services: compose_file.services.len(),
            total_networks: compose_file.networks.as_ref().map(|n| n.len()).unwrap_or(0),
            total_volumes: compose_file.volumes.as_ref().map(|v| v.len()).unwrap_or(0),
            compose_version: compose_file.version.clone(),
            validation_time_ms: validation_time,
        };

        ValidationReport {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            suggestions,
            summary,
        }
    }

    fn validate_version(
        &self,
        compose_file: &ComposeFile,
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
    ) {
        let version = &compose_file.version;

        if !self.is_valid_compose_version(version) {
            errors.push(ValidationError {
                service: None,
                field: Some("version".to_string()),
                message: format!("Invalid compose version: {}", version),
                error_type: ErrorType::InvalidVersion,
            });
            return;
        }

        if let Some(target) = &self.target_version {
            if version != target {
                warnings.push(ValidationWarning {
                    service: None,
                    field: Some("version".to_string()),
                    message: format!("Version {} differs from target version {}", version, target),
                    warning_type: WarningType::PortabilityIssue,
                });
            }
        }

        if self.is_deprecated_version(version) {
            warnings.push(ValidationWarning {
                service: None,
                field: Some("version".to_string()),
                message: format!("Compose version {} is deprecated", version),
                warning_type: WarningType::DeprecatedFeature,
            });
        }
    }

    fn validate_services(
        &self,
        services: &HashMap<String, Service>,
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
        suggestions: &mut Vec<ValidationSuggestion>,
    ) {
        for (service_name, service) in services {
            self.validate_service(service_name, service, errors, warnings, suggestions);
        }
    }

    fn validate_service(
        &self,
        service_name: &str,
        service: &Service,
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
        suggestions: &mut Vec<ValidationSuggestion>,
    ) {
        if service.image.is_empty() {
            errors.push(ValidationError {
                service: Some(service_name.to_string()),
                field: Some("image".to_string()),
                message: "Service must specify an image".to_string(),
                error_type: ErrorType::MissingRequiredField,
            });
        }

        if service.image.contains("latest") {
            warnings.push(ValidationWarning {
                service: Some(service_name.to_string()),
                field: Some("image".to_string()),
                message: "Using 'latest' tag is not recommended for production".to_string(),
                warning_type: WarningType::BestPracticeViolation,
            });
        }

        if let Some(network_mode) = &service.network_mode {
            if network_mode == "host" {
                warnings.push(ValidationWarning {
                    service: Some(service_name.to_string()),
                    field: Some("network_mode".to_string()),
                    message: "Host networking mode reduces container isolation".to_string(),
                    warning_type: WarningType::SecurityRisk,
                });
            }
        }

        if let Some(cap_add) = &service.cap_add {
            if cap_add.contains(&"SYS_ADMIN".to_string()) || cap_add.contains(&"ALL".to_string()) {
                warnings.push(ValidationWarning {
                    service: Some(service_name.to_string()),
                    field: Some("cap_add".to_string()),
                    message: "Adding privileged capabilities poses security risks".to_string(),
                    warning_type: WarningType::SecurityRisk,
                });
            }
        }

        if service.restart.is_none() {
            suggestions.push(ValidationSuggestion {
                service: Some(service_name.to_string()),
                field: Some("restart".to_string()),
                message: "Consider adding a restart policy for better resilience".to_string(),
                suggestion_type: SuggestionType::BestPractice,
            });
        }

        if service.healthcheck.is_none() {
            suggestions.push(ValidationSuggestion {
                service: Some(service_name.to_string()),
                field: Some("healthcheck".to_string()),
                message: "Consider adding a healthcheck for better monitoring".to_string(),
                suggestion_type: SuggestionType::BestPractice,
            });
        }

        if let Some(ports) = &service.ports {
            for port in ports {
                if port.starts_with("0.0.0.0:") {
                    suggestions.push(ValidationSuggestion {
                        service: Some(service_name.to_string()),
                        field: Some("ports".to_string()),
                        message: "Binding to 0.0.0.0 exposes ports to all interfaces".to_string(),
                        suggestion_type: SuggestionType::SecurityImprovement,
                    });
                    break;
                }
            }
        }

        if let Some(volumes) = &service.volumes {
            for volume in volumes {
                if volume.contains(":/") && !volume.contains(":ro") {
                    suggestions.push(ValidationSuggestion {
                        service: Some(service_name.to_string()),
                        field: Some("volumes".to_string()),
                        message: "Consider making bind mounts read-only when possible".to_string(),
                        suggestion_type: SuggestionType::SecurityImprovement,
                    });
                    break;
                }
            }
        }

        if let Some(deploy) = &service.deploy {
            if let Some(resources) = &deploy.resources {
                if let Some(limits) = &resources.limits {
                    if limits.memory.is_none() {
                        suggestions.push(ValidationSuggestion {
                            service: Some(service_name.to_string()),
                            field: Some("deploy.resources.limits.memory".to_string()),
                            message:
                                "Consider setting memory limits to prevent resource exhaustion"
                                    .to_string(),
                            suggestion_type: SuggestionType::PerformanceOptimization,
                        });
                    }
                }
            }
        }
    }

    fn check_best_practices_compliance(
        &self,
        compose_file: &ComposeFile,
        _warnings: &mut Vec<ValidationWarning>,
        suggestions: &mut Vec<ValidationSuggestion>,
    ) {
        let service_count = compose_file.services.len();

        if service_count > 20 {
            suggestions.push(ValidationSuggestion {
                service: None,
                field: None,
                message:
                    "Consider splitting large compose files into smaller, more manageable files"
                        .to_string(),
                suggestion_type: SuggestionType::BestPractice,
            });
        }

        let services_without_labels = compose_file
            .services
            .iter()
            .filter(|(_, service)| service.labels.is_none())
            .count();

        if services_without_labels > service_count / 2 {
            suggestions.push(ValidationSuggestion {
                service: None,
                field: None,
                message: "Consider adding descriptive labels to services for better organization"
                    .to_string(),
                suggestion_type: SuggestionType::BestPractice,
            });
        }

        if compose_file.networks.is_none() && service_count > 1 {
            suggestions.push(ValidationSuggestion {
                service: None,
                field: None,
                message: "Consider defining custom networks for better service isolation"
                    .to_string(),
                suggestion_type: SuggestionType::SecurityImprovement,
            });
        }
    }

    fn is_valid_compose_version(&self, version: &str) -> bool {
        matches!(
            version,
            "3.0" | "3.1" | "3.2" | "3.3" | "3.4" | "3.5" | "3.6" | "3.7" | "3.8" | "3.9"
        )
    }

    fn is_deprecated_version(&self, version: &str) -> bool {
        matches!(version, "3.0" | "3.1" | "3.2" | "3.3")
    }
}

pub fn format_validation_report(report: &ValidationReport, format: &str) -> Result<String> {
    match format.to_lowercase().as_str() {
        "json" => Ok(serde_json::to_string_pretty(report)?),
        "yaml" => Ok(serde_yaml::to_string(report)?),
        "text" => Ok(format_text_report(report)),
        _ => Err(AutoComposeError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Unsupported format: {}", format),
        ))),
    }
}

fn format_text_report(report: &ValidationReport) -> String {
    let mut output = String::new();

    output.push_str("=== Validation Report ===\n");
    output.push_str(&format!(
        "Status: {}\n",
        if report.is_valid { "VALID" } else { "INVALID" }
    ));
    output.push_str(&format!("Services: {}\n", report.summary.total_services));
    output.push_str(&format!("Networks: {}\n", report.summary.total_networks));
    output.push_str(&format!("Volumes: {}\n", report.summary.total_volumes));
    output.push_str(&format!("Version: {}\n", report.summary.compose_version));
    output.push_str(&format!(
        "Validation time: {}ms\n\n",
        report.summary.validation_time_ms
    ));

    if !report.errors.is_empty() {
        output.push_str("🚨 ERRORS:\n");
        for error in &report.errors {
            output.push_str(&format!(
                "  - {}\n",
                format_issue(&error.service, &error.field, &error.message)
            ));
        }
        output.push('\n');
    }

    if !report.warnings.is_empty() {
        output.push_str("⚠️  WARNINGS:\n");
        for warning in &report.warnings {
            output.push_str(&format!(
                "  - {}\n",
                format_issue(&warning.service, &warning.field, &warning.message)
            ));
        }
        output.push('\n');
    }

    if !report.suggestions.is_empty() {
        output.push_str("💡 SUGGESTIONS:\n");
        for suggestion in &report.suggestions {
            output.push_str(&format!(
                "  - {}\n",
                format_issue(&suggestion.service, &suggestion.field, &suggestion.message)
            ));
        }
        output.push('\n');
    }

    if report.is_valid && report.warnings.is_empty() && report.suggestions.is_empty() {
        output.push_str("✅ Your Docker Compose file looks great!\n");
    }

    output
}

fn format_issue(service: &Option<String>, field: &Option<String>, message: &str) -> String {
    match (service, field) {
        (Some(service), Some(field)) => format!("[{}:{}] {}", service, field, message),
        (Some(service), None) => format!("[{}] {}", service, message),
        (None, Some(field)) => format!("[{}] {}", field, message),
        (None, None) => message.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Service;

    #[test]
    fn test_validator_creation() {
        let validator = Validator::new(true, Some("3.9".to_string()));
        assert!(validator.check_best_practices);
        assert_eq!(validator.target_version, Some("3.9".to_string()));
    }

    #[test]
    fn test_version_validation() {
        let validator = Validator::new(false, None);
        assert!(validator.is_valid_compose_version("3.9"));
        assert!(!validator.is_valid_compose_version("4.0"));
        assert!(validator.is_deprecated_version("3.0"));
        assert!(!validator.is_deprecated_version("3.9"));
    }

    #[test]
    fn test_service_validation() {
        let validator = Validator::new(true, None);
        let mut services = HashMap::new();

        let service = Service {
            image: "nginx:latest".to_string(),
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
            init: None,
            privileged: None,
            tty: None,
            stdin_open: None,
            depends_on: None,
        };

        services.insert("test".to_string(), service);

        let compose_file = ComposeFile {
            version: "3.9".to_string(),
            services,
            networks: None,
            volumes: None,
        };

        let report = validator.validate_compose_object(&compose_file);
        assert!(report.is_valid);
        assert!(!report.warnings.is_empty()); // Should warn about 'latest' tag
    }
}
