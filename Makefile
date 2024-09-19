# Makefile for docker-autocompose and podman-autocompose

# Variables
CARGO = cargo
RUSTDOC = rustdoc
DOCKER_BIN = docker-autocompose
PODMAN_BIN = podman-autocompose
DOC_DIR = target/doc
# Default install directory, can be overridden
INSTALL_DIR ?= $(HOME)/.local/bin

# Default target
all: build doc

# Build the project
build:
	@echo "Building project..."
	@$(CARGO) build --release

# Run the docker version
run-docker:
	@echo "Running docker-autocompose..."
	@$(CARGO) run --release --bin $(DOCKER_BIN)

# Run the podman version
run-podman:
	@echo "Running podman-autocompose..."
	@$(CARGO) run --release --bin $(PODMAN_BIN)

# Build documentation
doc:
	@echo "Generating documentation..."
	@$(CARGO) doc --no-deps

# Open documentation in browser
doc-open: doc
	@echo "Opening documentation in browser..."
	@$(CARGO) doc --open

# Clean build artifacts and documentation
clean:
	@echo "Cleaning project..."
	@$(CARGO) clean

# Check code without building
check:
	@echo "Checking code..."
	@$(CARGO) check

# Run tests
test:
	@echo "Running tests..."
	@$(CARGO) test

# Format code
fmt:
	@echo "Formatting code..."
	@$(CARGO) fmt

# Lint code
lint:
	@echo "Linting code..."
	@$(CARGO) clippy

# Install both binaries
install: build
	@echo "Installing binaries to $(INSTALL_DIR)..."
	@mkdir -p $(INSTALL_DIR)
	@cp target/release/$(DOCKER_BIN) $(INSTALL_DIR)/
	@cp target/release/$(PODMAN_BIN) $(INSTALL_DIR)/
	@echo "Installation complete. Make sure $(INSTALL_DIR) is in your PATH."

# Uninstall both binaries
uninstall:
	@echo "Uninstalling binaries from $(INSTALL_DIR)..."
	@rm -f $(INSTALL_DIR)/$(DOCKER_BIN)
	@rm -f $(INSTALL_DIR)/$(PODMAN_BIN)
	@echo "Uninstallation complete."

# Help target
help:
	@echo "Available targets:"
	@echo "  all          - Build the project and generate documentation"
	@echo "  build        - Build both binaries in release mode"
	@echo "  run-docker   - Run the docker-autocompose binary"
	@echo "  run-podman   - Run the podman-autocompose binary"
	@echo "  doc          - Generate documentation"
	@echo "  doc-open     - Generate documentation and open in browser"
	@echo "  clean        - Clean build artifacts and documentation"
	@echo "  check        - Check code without building"
	@echo "  test         - Run tests"
	@echo "  fmt          - Format code"
	@echo "  lint         - Lint code using clippy"
	@echo "  install      - Install both binaries (default: $(INSTALL_DIR))"
	@echo "  uninstall    - Uninstall both binaries"
	@echo "  help         - Show this help message"
	@echo ""
	@echo "You can specify a custom install directory by setting INSTALL_DIR:"
	@echo "  make install INSTALL_DIR=/custom/path"

.PHONY: all build run-docker run-podman doc doc-open clean check test fmt lint install uninstall help
