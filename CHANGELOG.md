# Changelog

This document lists the changes made to the **autocompose-podman-docker** project.

---

## Release Builds

### Available Platforms

- **windows-x64**: `autocompose-windows-x64.zip`
- **linux-x64**: `autocompose-linux-x64.tar.gz`
- **linux-arm64**: `autocompose-linux-arm64.tar.gz`
- **linux-x64-musl**: `autocompose-linux-x64-musl.tar.gz`
- **linux-armv7**: `autocompose-linux-armv7.tar.gz`

---

## Recent Changes

- **Version 1.5.0** *(2025-01-30)*
  Major update with full feature integration and bug fixes:
  
  **Configuration System**
  - Fixed configuration file loading and application
  - Implemented proper configuration override chain: CLI args > config file > defaults
  - Added all missing configuration keys to the config set command
  - Added helpful error messages with suggestions for incorrect configuration keys
  
  **New Command-Line Options**
  - Added `--filter` (`-f`): Filter containers by pattern with wildcard support
  - Added `--exclude`: Exclude containers matching patterns
  - Added `--exclude-system`: Exclude system containers
  - Added `--docker-host`: Connect to specific Docker host
  - Added `--context`: Full Docker context support (reads from ~/.docker/contexts/)
  - Added `--include-networks`: Include network definitions in output
  - Added `--include-volumes`: Include volume definitions in output
  - Added `--debug`: Enable debug output
  - Added `--verbose` (`-v`): Set verbosity level (can be used multiple times)
  - Added `--preview`: Preview output without writing to file
  - Added `--compact`: Generate compact output
  - Added `--running-only`: Only process running containers (Podman)
  - Added `--strict`: Strict validation mode
  - Added short form `-v` for version option
  
  **Interactive Mode**
  - Fixed `-i` interactive mode in main binary
  - Added full interactive container selection for both Docker and Podman
  - Interactive mode now shows container name, image, ID, and state
  
  **Filtering and Pattern Matching**
  - Implemented comprehensive filtering functions for Docker and Podman
  - Added regex-based wildcard pattern matching (* and ?)
  - Filters work on container names, images, and IDs
  - Support for multiple filter patterns
  
  **Docker Context Support**
  - Implemented full Docker context reading and switching
  - Reads contexts from ~/.docker/contexts/meta/
  - Supports "current" context from Docker CLI config
  - Handles DOCKER_HOST environment variable for default context
  - SHA-256 hashing for context directory names
  
  **Output Formats**
  - Fixed TOML format generation
  - Added compact output mode for all formats (YAML, JSON, TOML)
  - Proper formatting based on selected output format
  
  **Debug and Logging**
  - Added multi-level debug output (DEBUG, INFO, TRACE)
  - Debug messages show configuration loading, filtering decisions, and processing steps
  - Verbose mode increases detail level progressively
  
  **Bug Fixes**
  - Fixed unused variable warnings
  - Fixed type mismatches with version field
  - Fixed compilation warnings with deprecated Bollard APIs
  - Fixed configuration not being applied to container processing
  - Fixed interactive mode not working in main binary

- **Commit pending** *(2025-06-16)*
  Fixed documentation code block formatting - Added proper white-space handling (pre-wrap) to ensure code blocks display line breaks correctly

- **Commit `9f347d3`** *(2025-04-14)*
  Change container information

- **Commit `3fc727f`** *(2025-04-14)*
  chore(deps): bump tokio from 1.44.1 to 1.44.2 (#18)

- **Commit `24b6082`** *(2025-04-14)*
  chore(deps): bump clap from 4.5.34 to 4.5.36 (#21)

- **Commit `f87467b`** *(2025-04-14)*
  chore(deps): bump tokio from 1.44.1 to 1.44.2 (#20)

- **Commit `674315d`** *(2025-03-31)*
  chore(deps): bump tokio from 1.44.0 to 1.44.1 (#16)

- **Commit `e5eff2f`** *(2025-03-31)*
  chore(deps): bump clap from 4.5.31 to 4.5.34 (#17)

- **Commit `63e0a0e`** *(2025-03-12)*
  chore(deps): bump serde from 1.0.218 to 1.0.219 (#12)

- **Commit `ad117b4`** *(2025-03-12)*
  chore(deps): bump serde_json from 1.0.139 to 1.0.140 (#13)

- **Commit `d46aba9`** *(2025-03-12)*
  chore(deps): bump tokio from 1.43.0 to 1.44.0 (#14)

- **Commit `1d22fd3`** *(2025-03-03)*
  chore(deps): bump clap from 4.5.30 to 4.5.31 (#11)

- **Commit `a3edecf`** *(2025-02-27)*
  chore(deps): bump serde from 1.0.217 to 1.0.218 (#9)

- **Commit `75a0cc6`** *(2025-02-27)*
  chore(deps): bump serde_json from 1.0.138 to 1.0.139 (#10)

- **Commit `120d70b`** *(2025-02-20)*
  adds rc1 (#7) (#8)

- **Commit `815bd93`** *(2025-01-31)*
  Merge pull request #5 from Drasrax/dependabot/cargo/main/serde_json-1.0

- **Commit `9f0dafe`** *(2025-01-31)*
  Merge pull request #6 from Drasrax/dependabot/cargo/main/bollard-0.18.1

- **Commit `0b5ebbf`** *(2025-01-13)*
  chore(deps): update bollard requirement from 0.11.0 to 0.18.1

- **Commit `8c08b20`** *(2025-01-13)*
  chore(deps): update serde_json requirement from 0.9 to 1.0

- **Commit `e12abf7`** *(2024-11-06)*
  Update SECURITY.md

- **Commit `b0162f1`** *(2024-11-06)*
  Update SECURITY.md

- **Commit `d264668`** *(2024-10-28)*
  Create SECURITY.md

- **Commit `e7d0f57`** *(2024-09-26)*
  Update dependabot.yml

- **Commit `bb11cf5`** *(2024-09-26)*
  Create dependabot.yml

- **Commit `aa40725`** *(2024-09-26)*
  Update README.md

- **Commit `f180a0b`** *(2024-09-26)*
  Update README.md

- **Commit `92786a1`** *(2024-09-26)*
  Update LICENSE

- **Commit `7836ed5`** *(2024-09-19)*
  Initial commit

### Docker Exporter (Bollard)

- **Performance & Concurrency**
  - Replaced sequential container inspection with parallel execution using Tokio and `FuturesUnordered`.
  - Refactored the code by extracting container inspection logic into dedicated functions for better modularity.

- **CLI Features**
  - Integrated Clap to handle command-line arguments (output file name, docker-compose version, etc.).

- **Docker Compose Export**
  - Fully transformed Docker container data into a Docker Compose configuration.
  - Supported common parameters: image, command, entrypoint, environment, ports, volumes, restart, etc.

- **Bug Fixes & Improvements**
  - Resolved compilation issues related to the use of `futures` and the `Send` trait.
  - Optimized YAML serialization to avoid outputting fields like `volumes: null` or `networks: null`.

---

### Podman Exporter

- **Advanced Export to Docker Compose**
  - Deep translation of `podman inspect` output into a Docker Compose configuration.
  - Supported parameters include:
    - **Basic Configuration:** image, command, entrypoint, environment, ports, volumes.
    - **Restart & Network:** restart policies, network mode, and detailed extraction of network settings (including IP addresses).
    - **Advanced Options:** logging, user, working directory, hostname, devices.
    - **Capabilities & Security:** handling of `cap_add` and `cap_drop`, security options, ulimits, sysctls.
    - **DNS:** support for DNS settings, DNS search, and extra_hosts.
    - **Healthcheck:** support for test commands, interval, timeout, retries, and start_period.
    - **Resource Deployment:** extraction and conversion of CPU and memory limits into the `deploy` section.

- **Network Management**
  - Extracted network configuration from `NetworkSettings.Networks` to assign IP addresses to services.
  - Accumulated a global network configuration for the top-level `networks` section using IPAM, by computing the subnet from the gateway and prefix.
  - Resolved the conflict between `network_mode` and `networks`: if `HostConfig.NetworkMode` is set and not `"default"`, only `network_mode` is used; otherwise, detailed network settings are extracted.

- **Image Resolution**
  - Added an asynchronous function `get_image_repo` to resolve an image hash to a full repository tag (e.g., `"ubuntu:latest"`) by executing `podman image inspect`.

- **Miscellaneous Improvements**
  - Filtered out unwanted labels (e.g., those starting with `io.buildah`) to keep the final file clean.
  - Used `#[serde(skip_serializing_if = "Option::is_none")]` to prevent printing empty fields.
  - Overall improved code robustness and readability through optimized asynchronous processing.

