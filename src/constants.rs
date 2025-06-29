/*!
Copyright (c) 2025 Olympus Chain SAS

This software is licensed under the Olympus Chain Internal Source License (OCISL).
You may read and modify this code for personal or internal non-commercial use only.
Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

Contact: contact@olympus-chain.fr
*/

// Memory conversion
pub const BYTES_PER_MB: u64 = 1024 * 1024;

// Time conversions
pub const NS_PER_SECOND: i64 = 1_000_000_000;
pub const SECONDS_PER_MINUTE: i64 = 60;
pub const SECONDS_PER_HOUR: i64 = 3600;

// String lengths
pub const IMAGE_HASH_LENGTH: usize = 64;
pub const SHORT_CONTAINER_ID_LENGTH: usize = 12;

// Limits
pub const LARGE_COMPOSE_SERVICE_COUNT: usize = 20;
pub const MAX_CONCURRENT_CONTAINERS: usize = 10;

// Container ID validation
pub const MAX_CONTAINER_ID_LENGTH: usize = 64;

// Image ID validation
pub const MAX_IMAGE_ID_LENGTH: usize = 256;

// CPU configuration
pub const CPU_SHARES_PER_CPU: u64 = 1024;
pub const NANO_CPUS_PER_CPU: u64 = 1_000_000_000;
