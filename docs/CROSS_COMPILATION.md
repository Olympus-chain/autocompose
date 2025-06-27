# Cross-Compilation Guide

This guide explains how to cross-compile the autocompose project for different platforms and architectures.

## Supported Targets

The project supports cross-compilation for the following targets:

- `x86_64-unknown-linux-gnu` - Linux x86_64 (GNU libc)
- `x86_64-unknown-linux-musl` - Linux x86_64 (MUSL libc - static linking)
- `aarch64-unknown-linux-gnu` - Linux ARM64 (GNU libc)
- `aarch64-unknown-linux-musl` - Linux ARM64 (MUSL libc - static linking)
- `armv7-unknown-linux-gnueabihf` - Linux ARMv7 (32-bit ARM with hardware float)
- `x86_64-pc-windows-gnu` - Windows x86_64 (MinGW)
- `x86_64-apple-darwin` - macOS x86_64
- `aarch64-apple-darwin` - macOS ARM64 (Apple Silicon)

## Prerequisites

1. **Rust toolchain**: Install Rust from [rustup.rs](https://rustup.rs/)

2. **Cross-compilation tools**: 
   ```bash
   # Install cross (recommended for Linux cross-compilation)
   cargo install cross
   ```

3. **Target-specific dependencies**:
   - For Windows targets: `mingw-w64`
   - For ARM targets: `gcc-arm-linux-gnueabihf`, `gcc-aarch64-linux-gnu`
   - For MUSL targets: `musl-tools`

## Quick Start

### Using the build script

The easiest way to cross-compile is using the provided script:

```bash
./cross-build.sh
```

This will:
- Install necessary tools
- Build for all supported targets
- Create distributable archives in the `dist/` directory

### Using Make

For specific targets:

```bash
# Build for a specific target
make build-cross-x86_64-pc-windows-gnu

# Build for all targets
make build-cross-all

# Package all builds
make package-cross
```

### Manual compilation

For manual control:

```bash
# Add the target
rustup target add x86_64-pc-windows-gnu

# Build with cargo
cargo build --release --target x86_64-pc-windows-gnu

# Or with cross (recommended)
cross build --release --target x86_64-pc-windows-gnu
```

## Configuration

The cross-compilation configuration is defined in:

1. **Cargo.toml**: Contains linker configurations for each target
2. **Makefile**: Provides convenient build targets
3. **cross-build.sh**: Automated build script

## Output Structure

After building, binaries will be organized as:

```
dist/
├── x86_64-unknown-linux-gnu/
│   ├── autocompose
│   ├── docker-autocompose-v2
│   └── podman-autocompose-v2
├── x86_64-pc-windows-gnu/
│   ├── autocompose.exe
│   ├── docker-autocompose-v2.exe
│   └── podman-autocompose-v2.exe
└── autocompose-x86_64-unknown-linux-gnu.tar.gz
    ...
```

## Troubleshooting

### Missing linker

If you get linker errors, install the appropriate cross-compilation toolchain:

```bash
# Ubuntu/Debian
sudo apt-get install gcc-mingw-w64-x86-64  # For Windows
sudo apt-get install gcc-aarch64-linux-gnu # For ARM64

# Fedora
sudo dnf install mingw64-gcc
sudo dnf install gcc-aarch64-linux-gnu
```

### macOS targets from Linux

Cross-compiling for macOS from Linux requires additional setup:
- Consider using [osxcross](https://github.com/tpoechtrager/osxcross)
- Or build on a macOS machine/CI

### Static linking with MUSL

MUSL targets produce statically linked binaries that work on any Linux distribution:

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

## CI/CD Integration

For automated builds, consider using GitHub Actions or similar CI services that provide multiple platform runners.