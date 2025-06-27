#Copyright (c) 2025 Olympus Chain SAS

#This software is licensed under the Olympus Chain Internal Source License (OCISL).
#You may read and modify this code for personal or internal non-commercial use only.
#Commercial use, redistribution, or reuse in other software is prohibited without prior written permission.

#Contact: contact@olympus-chain.fr


# Variables
CARGO = cargo
RUSTDOC = rustdoc
DOCKER_BIN = docker-autocompose
PODMAN_BIN = podman-autocompose
DOC_DIR = target/doc
INSTALL_DIR ?= $(HOME)/.local/bin

TARGET_x86_64-unknown-linux-gnu = linux-x64
TARGET_x86_64-unknown-linux-musl = linux-x64-musl
TARGET_aarch64-unknown-linux-musl = linux-arm64-musl
TARGET_x86_64-pc-windows-gnu = windows-x64

# targets list
TARGETS = x86_64-unknown-linux-gnu x86_64-unknown-linux-musl aarch64-unknown-linux-musl x86_64-pc-windows-gnu

BINARIES = autocompose docker-autocompose-v2 podman-autocompose-v2

DIST_DIR = dist

all: build doc

build:
	@echo "Building project..."
	@$(CARGO) build --release

run-docker:
	@echo "Running docker-autocompose..."
	@$(CARGO) run --release --bin $(DOCKER_BIN)

run-podman:
	@echo "Running podman-autocompose..."
	@$(CARGO) run --release --bin $(PODMAN_BIN)

doc:
	@echo "Generating documentation..."
	@$(CARGO) doc --no-deps

doc-open: doc
	@echo "Opening documentation in browser..."
	@$(CARGO) doc --open

clean:
	@echo "Cleaning project..."
	@$(CARGO) clean

check:
	@echo "Checking code..."
	@$(CARGO) check

# Run tests
test:
	@echo "Running tests..."
	@$(CARGO) test

fmt:
	@echo "Formatting code..."
	@$(CARGO) fmt

lint:
	@echo "Linting code..."
	@$(CARGO) clippy

install: build
	@echo "Installing binaries to $(INSTALL_DIR)..."
	@mkdir -p $(INSTALL_DIR)
	@cp target/release/$(DOCKER_BIN) $(INSTALL_DIR)/
	@cp target/release/$(PODMAN_BIN) $(INSTALL_DIR)/
	@echo "Installation complete. Make sure $(INSTALL_DIR) is in your PATH."

uninstall:
	@echo "Uninstalling binaries from $(INSTALL_DIR)..."
	@rm -f $(INSTALL_DIR)/$(DOCKER_BIN)
	@rm -f $(INSTALL_DIR)/$(PODMAN_BIN)
	@echo "Uninstallation complete."

build-cross-%:
	@echo "Building for target $*..."
	@$(CARGO) build --release --target $*

build-cross-all:
	@echo "Building for all cross-compilation targets..."
	@for target in $(TARGETS); do \
		echo "Building for $$target..."; \
		$(CARGO) build --release --target $$target || echo "Failed to build for $$target"; \
	done

release: clean-dist
	@echo "Creating release packages for all platforms..."
	@mkdir -p $(DIST_DIR)
	@for target in $(TARGETS); do \
		$(MAKE) cross-$$target || echo "Failed to build $$target"; \
	done
	@$(MAKE) create-release-notes
	@echo "Distribution structure:"
	@ls -la $(DIST_DIR)/

cross-%: check-cross
	$(eval PLATFORM := $(TARGET_$*))
	@if [ -z "$(PLATFORM)" ]; then \
		echo "Unknown target: $*"; \
		echo "Available targets: $(TARGETS)"; \
		exit 1; \
	fi
	@echo "Building for $* ($(PLATFORM))..."
	@rustup target add $* 2>/dev/null || echo "Could not add target $*"
	@if cross build --release --target $*; then \
		echo "Successfully built for $*"; \
		mkdir -p "$(DIST_DIR)/$(PLATFORM)"; \
		for bin in $(BINARIES); do \
			if [ "$*" = "x86_64-pc-windows-gnu" ]; then \
				if [ -f "target/$*/release/$$bin.exe" ]; then \
					cp "target/$*/release/$$bin.exe" "$(DIST_DIR)/$(PLATFORM)/$$bin.exe"; \
					echo "Copied $$bin.exe to $(PLATFORM)"; \
				fi; \
			else \
				if [ -f "target/$*/release/$$bin" ]; then \
					cp "target/$*/release/$$bin" "$(DIST_DIR)/$(PLATFORM)/$$bin"; \
					chmod +x "$(DIST_DIR)/$(PLATFORM)/$$bin"; \
					echo "Copied $$bin to $(PLATFORM)"; \
				fi; \
			fi; \
		done; \
		cp CHANGELOG.md "$(DIST_DIR)/$(PLATFORM)/" 2>/dev/null || true; \
		$(MAKE) create-archive PLATFORM=$(PLATFORM) TARGET=$*; \
	else \
		echo "Failed to build for $*"; \
		exit 1; \
	fi

create-archive:
	@if [ -z "$(PLATFORM)" ] || [ -z "$(TARGET)" ]; then \
		echo "PLATFORM and TARGET must be set"; \
		exit 1; \
	fi
	@if echo "$(TARGET)" | grep -q "windows"; then \
		if command -v zip >/dev/null 2>&1; then \
			echo "Creating ZIP archive for $(PLATFORM)..."; \
			cd $(DIST_DIR) && zip -r "autocompose-$(PLATFORM).zip" "$(PLATFORM)" && cd ..; \
		else \
			echo "Warning: zip not installed, skipping ZIP creation"; \
		fi; \
	else \
		echo "Creating TAR.GZ archive for $(PLATFORM)..."; \
		cd $(DIST_DIR) && tar -czf "autocompose-$(PLATFORM).tar.gz" "$(PLATFORM)" && cd ..; \
	fi

check-cross:
	@if ! command -v cross >/dev/null 2>&1; then \
		echo "Installing cross..."; \
		cargo install cross; \
	fi

clean-dist:
	@echo "Cleaning distribution directory..."
	@rm -rf $(DIST_DIR)

create-release-notes:
	@echo "Creating release notes..."
	@echo "# Autocompose Release Builds" > $(DIST_DIR)/RELEASE_NOTES.md
	@echo "" >> $(DIST_DIR)/RELEASE_NOTES.md
	@echo "## Available Platforms" >> $(DIST_DIR)/RELEASE_NOTES.md
	@echo "" >> $(DIST_DIR)/RELEASE_NOTES.md
	@for target in $(TARGETS); do \
		platform=$$(eval echo \$$TARGET_$$target); \
		if [ -d "$(DIST_DIR)/$$platform" ]; then \
			if echo "$$target" | grep -q "windows"; then \
				echo "- **$$platform**: \`autocompose-$$platform.zip\`" >> $(DIST_DIR)/RELEASE_NOTES.md; \
			else \
				echo "- **$$platform**: \`autocompose-$$platform.tar.gz\`" >> $(DIST_DIR)/RELEASE_NOTES.md; \
			fi; \
		fi; \
	done
	@cp CHANGELOG.md $(DIST_DIR)/ 2>/dev/null || true

install-targets:
	@echo "Installing Rust targets for cross-compilation..."
	@for target in $(TARGETS); do \
		echo "Installing target: $$target"; \
		rustup target add $$target; \
	done
	@echo "All targets installed successfully!"
	@echo ""
	@echo "Note: For successful cross-compilation, you also need to install:"
	@echo "- cross: cargo install cross"
	@echo "- For Windows targets: mingw-w64"
	@echo "- For Linux musl targets: musl-tools"

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
	@echo "Cross-compilation targets:"
	@echo "  release               - Build and package releases for all supported platforms"
	@echo "  cross-<target>        - Cross-compile for specific target"
	@echo "  install-targets       - Install all Rust targets for cross-compilation"
	@echo "  clean-dist            - Clean distribution directory"
	@echo ""
	@echo "Available targets:"
	@for target in $(TARGETS); do \
		platform=$$(eval echo \$$TARGET_$$target); \
		echo "  - $$target ($$platform)"; \
	done
	@echo ""
	@echo "You can specify a custom install directory by setting INSTALL_DIR:"
	@echo "  make install INSTALL_DIR=/custom/path"

.PHONY: all build run-docker run-podman doc doc-open clean check test fmt lint install uninstall help build-cross-% build-cross-all release cross-% install-targets check-cross clean-dist create-release-notes create-archive
