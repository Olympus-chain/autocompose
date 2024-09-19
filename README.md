# Docker/Podman AutoCompose

Docker/Podman AutoCompose is a utility that automatically generates docker-compose.yml files from running containers, supporting both Docker and Podman environments.

## Features

- Generate docker-compose.yml files from running Docker containers
- Generate docker-compose.yml files from running Podman containers
- Easy-to-use command-line interface
- Support for complex container configurations

## Installation

### Prerequisites

- Rust programming language (latest stable version)
- Docker or Podman installed on your system

### Building from source

1. Clone the repository:
   ```
   git clone https://github.com/Drasrax/autocompose-podman-docker.git
   cd autocompose-podman-docker
   ```

2. Build the project:
   ```
   make build
   ```

3. Install the binaries:
   ```
   make install
   ```

   By default, this will install the binaries to `~/.local/bin`. You can specify a different installation directory:
   ```
   make install INSTALL_DIR=/custom/path
   ```

## Usage

### Docker AutoCompose

To generate a docker-compose.yml file for your running Docker containers:

```
docker-autocompose
```

### Podman AutoCompose

To generate a docker-compose.yml file for your running Podman containers:

```
podman-autocompose
```

Both commands will output the generated docker-compose.yml to the console. To save the output to a file, use:

```
docker-autocompose > docker-compose.yml
```

or

```
podman-autocompose > docker-compose.yml
```

## Configuration

Currently, the tool doesn't require any configuration files. It automatically detects and analyzes running containers in your Docker or Podman environment.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to the Docker and Podman teams for their excellent container technologies.
- This project uses the [bollard](https://github.com/fussybeaver/bollard) crate for Docker API interactions.


