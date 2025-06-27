# AutoCompose

<div align="center">

[![Version](https://img.shields.io/badge/version-2.0.0-blue.svg)](https://github.com/Olympus-chain/autocompose/releases)
[![License](https://img.shields.io/badge/license-OCISL-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.65+-orange.svg)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/docker-20.10+-blue.svg)](https://www.docker.com)
[![Podman](https://img.shields.io/badge/podman-3.0+-purple.svg)](https://podman.io)

**Automatically generate Docker Compose files from running containers**

[Documentation](docs/) | [Installation](#installation) | [Quick Start](#quick-start) | [Features](#features) | [Contributing](CONTRIBUTING.md)

</div>

---

## Overview

AutoCompose is a powerful command-line tool that reverse-engineers running Docker or Podman containers into clean, reusable `docker-compose.yml` files. Perfect for:

- 📦 **Migrating** existing container deployments to compose files
- 📝 **Documenting** running container configurations  
- 🔄 **Creating** reproducible development environments
- 🔍 **Auditing** container security settings
- 🎯 **Standardizing** deployment configurations

## Key Features

### 🚀 Core Capabilities

- **Dual Runtime Support**: Works seamlessly with both Docker and Podman
- **Comprehensive Extraction**: Captures all container configurations including networks, volumes, environment variables, and more
- **Security-First**: Automatically filters sensitive data like passwords and API keys
- **Smart Processing**: Resolves image digests to readable tags and sanitizes service names
- **Multiple Formats**: Export to YAML, JSON, or TOML
- **Built-in Validation**: Check compose files for errors, warnings, and best practices

### 🔧 Configuration Extraction

AutoCompose captures:
- Container metadata (image, name, hostname, working directory)
- Environment variables (with sensitive data filtering)
- Network configurations (ports, custom networks, DNS)
- Storage (volumes, bind mounts, tmpfs)
- Resource limits (CPU, memory)
- Security settings (capabilities, privileged mode)
- Health checks and restart policies
- Logging configuration

## Installation

### Requirements

- Linux, macOS, or Windows
- Docker Engine 20.10+ or Podman 3.0+
- Rust 1.65+ (for building from source)

### Pre-built Binaries

Download the latest release for your platform:

```bash
# Linux x64
curl -L https://github.com/Olympus-chain/autocompose/releases/latest/download/autocompose-linux-amd64 -o autocompose

# macOS (Intel)
curl -L https://github.com/Olympus-chain/autocompose/releases/latest/download/autocompose-darwin-amd64 -o autocompose

# macOS (Apple Silicon)
curl -L https://github.com/Olympus-chain/autocompose/releases/latest/download/autocompose-darwin-arm64 -o autocompose

# Make executable and install
chmod +x autocompose
sudo mv autocompose /usr/local/bin/
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/Olympus-chain/autocompose.git
cd autocompose

# Build with Cargo
cargo build --release

# Install
sudo cp target/release/autocompose /usr/local/bin/
```

### Cross-Platform Builds

See [CROSS_COMPILATION.md](docs/CROSS_COMPILATION.md) for building for other platforms (ARM, Windows, musl).

## Quick Start

### Basic Usage

```bash
# Generate compose file from all running Docker containers
autocompose docker

# Generate from specific Docker containers
autocompose docker nginx redis postgres

# Generate from Podman containers with custom output
autocompose podman -o podman-compose.yml

# Preview without writing to file
autocompose docker --dry-run

# Export as JSON
autocompose docker --format json -o services.json
```

### Validation

```bash
# Validate an existing compose file
autocompose validate docker-compose.yml

# Check with best practices recommendations
autocompose validate docker-compose.yml --check-best-practices

# Get validation report as JSON
autocompose validate docker-compose.yml --format json
```

### Configuration

```bash
# View current configuration
autocompose config show

# Set default compose version
autocompose config set default_compose_version 3.9

# Enable parallel processing
autocompose config set enable_parallel true

# Add exclusion pattern
autocompose config set exclude_patterns "test-*,dev-*"
```

## Advanced Usage

### Filtering and Selection

```bash
# Exclude containers by name pattern
autocompose docker --exclude "test-*" --exclude "dev-*"

# Include only specific networks
autocompose docker --network production

# Skip system containers
autocompose docker --skip-system
```

### Security Options

```bash
# Include sensitive environment variables (use with caution)
autocompose docker --include-sensitive

# Filter specific environment variables
autocompose docker --env-filter "API_*,SECRET_*"
```

### Performance Tuning

```bash
# Disable parallel processing for debugging
autocompose docker --no-parallel

# Increase connection timeout
autocompose docker --timeout 60
```

## Examples

### Multi-Service Application

```bash
# Generate compose file for a web application stack
autocompose docker nginx postgres redis rabbitmq -o webapp-stack.yml
```

### Development Environment

```bash
# Export development containers with local volume mounts
autocompose docker --include-volumes --network dev-network -o dev-compose.yml
```

### Production Migration

```bash
# Generate and validate production configuration
autocompose docker --skip-system -o prod-compose.yml
autocompose validate prod-compose.yml --check-best-practices
```

## Documentation

- 📚 [Full Documentation](docs/)
- 🌍 Available in multiple languages:
  - [English](docs/v1.5/en/README.md)
  - [Français](docs/v1.5/fr/README.md)
  - [Español](docs/v1.5/es/README.md)
  - [Deutsch](docs/v1.5/de/README.md)

## Contributing

We welcome contributions! See our [Contributing Guide](CONTRIBUTING.md) for details.

Priority areas for contribution:
- Container dependency detection algorithms
- Cloud provider integrations
- Additional output formats
- Performance optimizations
- Documentation and examples

## Roadmap

See [ROADMAP.md](ROADMAP.md) for planned features including:
- 🔗 Automatic dependency detection
- ☸️ Kubernetes manifest generation
- 🌐 Web UI interface
- 🔄 CI/CD integrations
- ☁️ Cloud provider support

## License

This project is licensed under the Olympus Chain Internal Source License (OCISL).
See [LICENSE](LICENSE) for details.

**Key points:**
- ✅ Free for personal and internal non-commercial use
- ❌ Commercial use requires a license
- ❌ Cannot be used in competing products

## Support

- 📖 [Documentation](docs/)
- 🐛 [Report Issues](https://github.com/Olympus-chain/autocompose/issues)
- 💬 [Discussions](https://github.com/Olympus-chain/autocompose/discussions)
- 📧 [Contact](mailto:contact@olympus-chain.fr)

---

<div align="center">

**Built with ❤️ by [Olympus Chain](https://olympus-chain.fr)**

Empowering developers with powerful container management tools

</div>