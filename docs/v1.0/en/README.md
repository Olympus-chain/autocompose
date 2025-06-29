# AutoCompose v1.0 Documentation

Welcome to AutoCompose v1.0! This is the original release that provides basic functionality for generating Docker Compose files from running containers.

> **Note:** This is a legacy version. For enhanced features, improved performance, and better container detection, please consider upgrading to [version 1.5](../../v1.5/en/index.html).

## What is AutoCompose?

AutoCompose is a command-line tool that automatically generates Docker Compose files from your running Docker containers. It simplifies the process of creating compose files by extracting configuration from existing containers.

### Key Features

- **Automatic Generation:** Creates docker-compose.yml from running containers
- **Basic Configuration:** Extracts essential container settings
- **Simple CLI:** Easy-to-use command-line interface
- **YAML Output:** Standard Docker Compose format

## Installation

### Requirements

- Linux or macOS
- Docker Engine 19.03+
- Python 3.6+ or pre-built binary

### Download Binary

```bash
# Download the v1.0 release
wget https://github.com/Olympus-chain/autocompose/releases/download/v1.0.0/autocompose
chmod +x autocompose
sudo mv autocompose /usr/local/bin/

# Verify installation
autocompose --version
```

### Install from Source

```bash
# Clone repository (v1.0 tag)
git clone --branch v1.0.0 https://github.com/Olympus-chain/autocompose.git
cd autocompose-podman-docker

# Install
make install
```

## Quick Start

### Basic Example

Generate a compose file from all running containers:

```bash
# Generate docker-compose.yml
autocompose

# Output to specific file
autocompose -o my-compose.yml

# Display output without saving
autocompose --stdout
```

### What Gets Exported

AutoCompose v1.0 extracts the following information:

- Container image and tag
- Container name
- Environment variables
- Port mappings
- Volume mounts
- Restart policy
- Networks (basic)

## Basic Usage

### Command Syntax

```bash
autocompose [OPTIONS]

Where OPTIONS can be:
  -o, --output FILE    Output file (default: docker-compose.yml)
  --stdout             Print to stdout instead of file
  -v, --version        Show version
  -h, --help           Show help message
```

### Simple Workflow

1. Start your containers manually with docker run
2. Configure them as needed
3. Run autocompose to generate the compose file
4. Use the generated file for future deployments

```bash
# Example: Running containers manually
docker run -d --name web -p 80:80 nginx
docker run -d --name db -e MYSQL_ROOT_PASSWORD=secret mysql:5.7

# Generate compose file
autocompose

# View the result
cat docker-compose.yml
```

## Command Options

### Available Options

| Option | Description | Default |
|--------|-------------|---------|
| `-o, --output` | Output file path | docker-compose.yml |
| `--stdout` | Print to standard output | false |
| `-v, --version` | Display version information | - |
| `-h, --help` | Show help message | - |

### Environment Variables

```bash
# Docker socket location (if non-standard)
export DOCKER_HOST=tcp://localhost:2375

# Run autocompose
autocompose
```

## Output Format

### Generated Structure

AutoCompose v1.0 generates a standard Docker Compose v2 file:

```yaml
version: '2'
services:
  container_name:
    image: image:tag
    container_name: container_name
    environment:
      - ENV_VAR=value
    ports:
      - "host:container"
    volumes:
      - /host/path:/container/path
    restart: policy
```

### Example Output

For a simple web application:

```yaml
version: '2'
services:
  webapp:
    image: nginx:latest
    container_name: webapp
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /var/www/html:/usr/share/nginx/html:ro
    restart: unless-stopped
    
  database:
    image: mysql:5.7
    container_name: database
    environment:
      - MYSQL_ROOT_PASSWORD=secretpass
      - MYSQL_DATABASE=myapp
    volumes:
      - /var/lib/mysql:/var/lib/mysql
    restart: always
```

## Examples

### Simple Web Server

```bash
# Run nginx container
docker run -d \
  --name webserver \
  -p 8080:80 \
  -v ~/website:/usr/share/nginx/html:ro \
  nginx:alpine

# Generate compose file
autocompose -o webserver-compose.yml

# Result:
version: '2'
services:
  webserver:
    image: nginx:alpine
    container_name: webserver
    ports:
      - "8080:80"
    volumes:
      - ~/website:/usr/share/nginx/html:ro
```

### Database Container

```bash
# Run PostgreSQL
docker run -d \
  --name postgres-db \
  -e POSTGRES_PASSWORD=mypassword \
  -e POSTGRES_DB=mydb \
  -v postgres-data:/var/lib/postgresql/data \
  -p 5432:5432 \
  postgres:12

# Generate compose file
autocompose

# Result includes environment variables and volumes
```

### Multi-Container Setup

```bash
# Run multiple containers
docker run -d --name frontend -p 3000:3000 my-app:frontend
docker run -d --name backend -p 5000:5000 --link frontend my-app:backend
docker run -d --name cache redis:alpine

# Generate complete compose file
autocompose -o full-stack.yml

# Creates compose file with all three services
```

## Troubleshooting

### Common Issues

#### No containers found

Ensure Docker daemon is running and containers are active:

```bash
# Check Docker status
docker info

# List running containers
docker ps

# If no containers running, start some first
docker run -d nginx
```

#### Permission denied

Add your user to the docker group or use sudo:

```bash
# Add user to docker group
sudo usermod -aG docker $USER

# Or run with sudo
sudo autocompose
```

#### Cannot connect to Docker

Check Docker socket:

```bash
# Default socket location
ls -la /var/run/docker.sock

# If using custom socket
export DOCKER_HOST=unix:///path/to/docker.sock
autocompose
```

### Getting Help

```bash
# Display help
autocompose --help

# Check version
autocompose --version

# Report issues
# https://github.com/Olympus-chain/autocompose/issues
```

## Limitations

### Known Limitations in v1.0

- **Docker Only:** No Podman support
- **Basic Extraction:** Limited configuration options extracted
- **No Filtering:** Exports all running containers
- **Networks:** Basic network support only
- **Compose Version:** Only supports version 2 format
- **No Validation:** Generated files are not validated

### Features Not Included

- Health checks
- Resource limits
- Custom network configurations
- Security options
- Labels
- Logging configuration

> **Upgrade Recommendation:** For these features and more, please upgrade to [AutoCompose v1.5](../../v1.5/en/index.html) which includes comprehensive container detection, advanced filtering, validation, and support for both Docker and Podman.

### Manual Adjustments

After generation, you may need to manually edit the compose file to:

- Add missing configuration options
- Define custom networks
- Set resource constraints
- Add health checks
- Configure logging