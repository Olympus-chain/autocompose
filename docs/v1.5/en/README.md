# AutoCompose v1.5 Documentation

Welcome to AutoCompose v1.5! This enhanced version brings improved Docker and Podman support, advanced filtering options, and performance optimizations.

> **Version 1.5 Highlights:** Enhanced container detection, improved network configuration extraction, better volume mapping, and optimized performance for large deployments.

## What is AutoCompose?

AutoCompose is a powerful command-line tool that automatically generates Docker Compose files from your running containers. It simplifies the process of converting existing container deployments into reproducible, version-controlled configurations.

### Key Benefits

- **Automation:** No manual YAML writing - extract configurations automatically
- **Accuracy:** Captures exact container configurations including networks, volumes, and environment variables
- **Flexibility:** Support for both Docker and Podman environments
- **Intelligence:** Smart filtering and validation ensure clean, optimized output

## Installation

### System Requirements

- Linux, macOS, or Windows (with WSL2)
- Docker Engine 20.10+ or Podman 3.0+
- Rust 1.65+ (for building from source)

### Pre-built Binaries

Download the latest release for your platform:

```bash
# Linux/macOS
curl -L https://github.com/Olympus-chain/autocompose/releases/latest/download/autocompose-linux-amd64 -o autocompose
chmod +x autocompose
sudo mv autocompose /usr/local/bin/

# Verify installation
autocompose --version
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/Olympus-chain/autocompose.git
cd autocompose-podman-docker

# Build with Cargo
cargo build --release

# Install
sudo cp target/release/autocompose /usr/local/bin/
```

### Docker Installation

You can also run AutoCompose using Docker:

```bash
# Create an alias for easy usage
alias autocompose='docker run --rm -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/drasrax/autocompose:latest'

# Use the tool
autocompose --help
```

## Quick Start

### Basic Export

Generate a Docker Compose file from all running containers:

```bash
# Export all running containers
autocompose

# Export to a specific file
autocompose -o my-stack.yml

# Preview without saving
autocompose --dry-run
```

### Filtered Export

Export specific containers based on criteria:

```bash
# Export containers matching a pattern
autocompose --filter "web-*"

# Export only running containers
autocompose --running-only

# Exclude system containers
autocompose --exclude-system
```

### Interactive Mode

Select containers interactively:

```bash
# Launch interactive selection
autocompose --interactive

# With preview
autocompose --interactive --preview
```

## Features

### Container Detection

AutoCompose v1.5 features enhanced container detection capabilities:

- Automatic detection of Docker and Podman containers
- Support for rootless Podman deployments
- Detection of container dependencies and links
- Intelligent grouping of related containers

### Configuration Extraction

Comprehensive extraction of container configurations:

| Category | Extracted Fields | v1.5 Enhancements |
|----------|------------------|-------------------|
| Basic Info | Image, name, command, working directory | Improved tag resolution |
| Networking | Ports, networks, hostname, DNS | IPv6 support, custom drivers |
| Storage | Volumes, bind mounts, tmpfs | Volume driver options |
| Runtime | Environment, labels, restart policy | Healthcheck configurations |
| Security | Capabilities, security options | SELinux contexts, AppArmor |

### Output Formats

Multiple output formats for different use cases:

```bash
# Standard YAML (default)
autocompose -o docker-compose.yml

# JSON format
autocompose --format json -o compose.json

# YAML with specific version
autocompose --compose-version 3.8 -o compose-v3.8.yml

# Compact output
autocompose --compact -o minimal.yml
```

## Basic Usage

### Command Structure

```bash
autocompose [OPTIONS] [CONTAINERS...]

OPTIONS:
    -o, --output <FILE>           Output file (default: docker-compose.yml)
    -f, --format <FORMAT>         Output format: yaml, json (default: yaml)
    -v, --compose-version <VER>   Compose file version (default: 3.8)
    --dry-run                     Preview without writing file
    --interactive                 Interactive container selection
    --help                        Display help information
```

### Common Workflows

#### Development Environment

Export your development stack:

```bash
# Export dev containers
autocompose --filter "dev-*" -o dev-compose.yml

# Include only specific services
autocompose dev-web dev-db dev-redis -o dev-stack.yml

# With custom network
autocompose --network dev-network -o dev-compose.yml
```

#### Production Migration

Prepare containers for production deployment:

```bash
# Export with production settings
autocompose --running-only \
  --exclude-system \
  --remove-caps \
  -o production-compose.yml

# Validate the output
autocompose validate production-compose.yml
```

## Docker Commands

### Docker-Specific Options

```bash
# Connect to remote Docker daemon
autocompose --docker-host tcp://remote:2375

# Use specific Docker context
autocompose --context production

# Include Docker labels
autocompose --include-labels

# Preserve container IDs
autocompose --preserve-ids
```

### Network Configuration

Advanced network extraction for Docker:

```bash
# Include custom networks
autocompose --include-networks

# Map network aliases
autocompose --preserve-aliases

# Include network driver options
autocompose --network-details
```

### Volume Management

```bash
# Include named volumes
autocompose --include-volumes

# Convert bind mounts to volumes
autocompose --convert-mounts

# Include volume driver options
autocompose --volume-details
```

## Podman Commands

### Podman-Specific Features

AutoCompose v1.5 includes enhanced Podman support:

```bash
# Rootless Podman
autocompose --podman-rootless

# Include pod configurations
autocompose --include-pods

# SystemD integration
autocompose --systemd-compatible

# SELinux labels
autocompose --preserve-selinux
```

### Pod Management

```bash
# Export entire pods
autocompose --pod my-app-pod

# Group by pods
autocompose --group-by-pod

# Include infra containers
autocompose --include-infra
```

## Filtering Options

### Name-Based Filtering

```bash
# Wildcard patterns
autocompose --filter "app-*"

# Regular expressions
autocompose --filter-regex "^(web|api)-.*"

# Multiple filters
autocompose --filter "web-*" --filter "api-*"

# Exclusion patterns
autocompose --exclude "test-*" --exclude "*-temp"
```

### State-Based Filtering

```bash
# Only running containers
autocompose --running-only

# Include stopped containers
autocompose --all

# By container state
autocompose --state running,paused

# By health status
autocompose --health healthy
```

### Label-Based Filtering

```bash
# Filter by label
autocompose --label-filter "environment=production"

# Multiple labels (AND)
autocompose --label-filter "app=myapp" --label-filter "tier=frontend"

# Label exists
autocompose --has-label "backup"

# Label pattern
autocompose --label-regex "version=2\.*"
```

## Configuration

### Configuration File

AutoCompose supports configuration files for persistent settings:

```bash
# Create default config
autocompose config init

# Location: ~/.autocompose/config.yml
# Edit with your preferred settings
```

### Configuration Options

```yaml
# config.yml example
defaults:
  output: docker-compose.yml
  format: yaml
  compose_version: "3.8"
  
filters:
  exclude_system: true
  exclude_patterns:
    - "k8s_*"
    - "*_test"
  
docker:
  socket: /var/run/docker.sock
  timeout: 30
  
podman:
  socket: /run/user/1000/podman/podman.sock
  rootless: true
  
output:
  compact: false
  sort_services: true
  include_timestamps: false
```

### Environment Variables

```bash
# Override config file
export AUTOCOMPOSE_CONFIG=/path/to/config.yml

# Docker socket
export DOCKER_HOST=tcp://localhost:2375

# Output directory
export AUTOCOMPOSE_OUTPUT_DIR=/compose-files

# Log level
export AUTOCOMPOSE_LOG_LEVEL=debug
```

## Validation

### Built-in Validation

AutoCompose v1.5 includes comprehensive validation:

```bash
# Validate generated file
autocompose validate docker-compose.yml

# Validate with specific version
autocompose validate -v 3.8 docker-compose.yml

# Strict validation
autocompose validate --strict docker-compose.yml

# Check for security issues
autocompose validate --security docker-compose.yml
```

### Validation Checks

- **Syntax:** YAML/JSON syntax validation
- **Schema:** Compose file schema compliance
- **References:** Network, volume, and service references
- **Security:** Privileged containers, capabilities, bind mounts
- **Best Practices:** Resource limits, health checks, restart policies

### Validation Output

```
# Example validation output
✓ Syntax valid
✓ Schema compliant (version 3.8)
⚠ Warning: Service 'web' uses tag 'latest'
⚠ Warning: Service 'db' missing health check
✗ Error: Network 'frontend' referenced but not defined
✗ Error: Volume 'data' has invalid driver options

Summary: 2 errors, 2 warnings
```

## Best Practices

### Security Recommendations

- Always review generated files before deployment
- Remove unnecessary privileges and capabilities
- Use specific image tags instead of 'latest'
- Implement proper secret management
- Set appropriate resource limits

### Performance Tips

- Use `--running-only` for faster processing
- Filter containers to reduce processing time
- Enable caching for repeated exports
- Use compact mode for smaller files

### Maintenance

- Version control your compose files
- Document custom modifications
- Regular validation of compose files
- Keep AutoCompose updated

## Troubleshooting

### Common Issues

#### Connection Errors

```bash
# Check Docker daemon
docker info

# Check socket permissions
ls -la /var/run/docker.sock

# Use sudo if needed
sudo autocompose

# Specify socket explicitly
autocompose --docker-socket /var/run/docker.sock
```

#### Permission Denied

```bash
# Add user to docker group
sudo usermod -aG docker $USER

# Logout and login again
# Or use newgrp
newgrp docker
```

#### Empty Output

```bash
# Check if containers are running
docker ps

# Include all containers
autocompose --all

# Check filters
autocompose --no-filters

# Enable debug logging
AUTOCOMPOSE_LOG_LEVEL=debug autocompose
```

### Debug Mode

```bash
# Enable debug output
autocompose --debug

# Verbose logging
autocompose -vvv

# Dry run with debug
autocompose --dry-run --debug

# Export debug information
autocompose debug-info > debug.txt
```

## CLI Reference

### Global Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--output` | `-o` | Output file path | docker-compose.yml |
| `--format` | `-f` | Output format (yaml/json) | yaml |
| `--compose-version` | `-v` | Compose file version | 3.8 |
| `--dry-run` | | Preview without writing | false |
| `--interactive` | `-i` | Interactive selection | false |
| `--help` | `-h` | Show help message | |
| `--version` | `-V` | Show version | |

### Filter Options

| Option | Description | Example |
|--------|-------------|---------|
| `--filter` | Filter by name pattern | `--filter "web-*"` |
| `--exclude` | Exclude by pattern | `--exclude "*-test"` |
| `--running-only` | Only running containers | `--running-only` |
| `--all` | Include stopped containers | `--all` |
| `--label-filter` | Filter by label | `--label-filter "env=prod"` |

## API Reference

### Library Usage

AutoCompose can be used as a library in Rust projects:

```rust
// Cargo.toml
[dependencies]
autocompose = "1.5"

// main.rs
use autocompose::{AutoCompose, Config, FilterOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    let filter = FilterOptions::new()
        .running_only(true)
        .exclude_pattern("test-*");
    
    let compose = AutoCompose::new(config)
        .with_filter(filter)
        .generate()?;
    
    println!("{}", compose.to_yaml()?);
    Ok(())
}
```

### Key Types

```rust
// Configuration
pub struct Config {
    pub docker_socket: String,
    pub compose_version: String,
    pub output_format: Format,
}

// Filter options
pub struct FilterOptions {
    pub patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub running_only: bool,
    pub labels: HashMap<String, String>,
}

// Generated compose
pub struct ComposeFile {
    pub version: String,
    pub services: HashMap<String, Service>,
    pub networks: Option<HashMap<String, Network>>,
    pub volumes: Option<HashMap<String, Volume>>,
}
```

## Examples

### Multi-Service Application

```yaml
# Running containers:
# - webapp (nginx)
# - api (node:14)
# - database (postgres:13)
# - cache (redis:6)

# Export full stack
autocompose -o fullstack.yml

# Result:
version: '3.8'
services:
  webapp:
    image: nginx:latest
    ports:
      - "80:80"
    volumes:
      - ./html:/usr/share/nginx/html
    networks:
      - frontend
    
  api:
    image: node:14
    working_dir: /app
    command: npm start
    environment:
      NODE_ENV: production
      DB_HOST: database
    ports:
      - "3000:3000"
    networks:
      - frontend
      - backend
    
  database:
    image: postgres:13
    environment:
      POSTGRES_DB: myapp
      POSTGRES_USER: user
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - db-data:/var/lib/postgresql/data
    networks:
      - backend
    
  cache:
    image: redis:6
    command: redis-server --appendonly yes
    volumes:
      - cache-data:/data
    networks:
      - backend

networks:
  frontend:
  backend:

volumes:
  db-data:
  cache-data:
```

### Development Environment

```bash
# Export with development overrides
autocompose \
  --filter "dev-*" \
  --include-labels \
  --preserve-mounts \
  -o docker-compose.dev.yml

# Result includes:
# - Source code bind mounts
# - Development environment variables
# - Debug ports exposed
# - No restart policies
```

### Production Deployment

```bash
# Strict production export
autocompose \
  --running-only \
  --exclude-system \
  --remove-caps \
  --add-healthchecks \
  --resource-limits \
  -o docker-compose.prod.yml

# Includes:
# - Health checks for all services
# - Resource limits (CPU/Memory)
# - Restart policies
# - No privileged containers
# - Specific image tags
```

## Changelog

### Version 1.5.0 (Current)

- **Enhanced Container Detection:** Improved detection of container relationships and dependencies
- **Advanced Filtering:** New regex filters, label-based filtering, and state filtering
- **Podman Improvements:** Better rootless support, pod configurations, SystemD integration
- **Network Enhancements:** IPv6 support, custom network drivers, preserve aliases
- **Performance:** 3x faster processing for large deployments, parallel container inspection
- **Validation:** Comprehensive validation with security checks and best practice recommendations
- **Output Formats:** Added JSON output, compact mode, sorted services

### Version 1.0.0

- Initial release
- Basic Docker support
- YAML output
- Simple filtering

## Roadmap

Our vision for AutoCompose is to become the most comprehensive and user-friendly tool for container-to-compose conversion. Here's what's coming next:

### Version 1.6 - Q3 2025

#### Core Enhancements

**🔗 Container Dependencies**
- Auto-detect `depends_on` relationships
- Health check-based dependencies
- Start order analysis

**💾 Volume Management**
- Proper volume definitions
- Volume driver options
- Backup recommendations

**🔍 Advanced Filtering**
- Label-based filtering
- Time-based filtering
- Resource-based filtering

**🌐 Network Enhancements**
- External networks support
- Advanced driver options
- IPv6 improvements

### Version 1.7 - Q4 2025

#### Advanced Features

**☸️ Kubernetes Integration**
- Pod to Compose conversion
- ConfigMaps & Secrets support
- Basic Helm chart conversion

**📋 Multi-Stage Compose**
- Environment-specific files
- Override management
- File merging capabilities

**🔨 Build Context Support**
- Dockerfile detection
- Build arguments
- Multi-stage builds

**🖥️ Web Interface**
- Interactive web UI
- Visual dependency editor
- Real-time preview

### Version 2.0 - Q1 2026

#### Enterprise Features

**🐝 Swarm Mode**
- Stack file generation
- Placement constraints
- Service replicas

**🔄 Bidirectional Sync**
- Compose to containers
- Live diff detection
- Update propagation

**🚀 CI/CD Integration**
- GitHub Actions
- GitLab CI templates
- Jenkins plugins

**📊 Monitoring**
- Prometheus labels
- Monitoring overlays
- Alert generation

### Version 2.1+ - 2026+

#### Future Vision

**🤖 AI-Powered**
- ML optimization
- Predictive scaling
- Anomaly detection

**☁️ Multi-Cloud**
- AWS ECS support
- Azure Container Instances
- Google Cloud Run

**🔌 Plugin System**
- Third-party extensions
- Plugin marketplace
- Custom processors

**🛠️ Developer Tools**
- IDE integrations
- Real-time linting
- Smart completion

### Want to Contribute?

We welcome contributions! Priority areas include:
- Container dependency detection algorithms
- Cloud provider integrations
- Documentation and examples
- Performance optimizations

🤝 [Contributing Guide](https://github.com/Olympus-chain/autocompose/blob/main/CONTRIBUTING.md)