# Changelog

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

