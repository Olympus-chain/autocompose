# Contributing to AutoCompose

Thank you for your interest in contributing to AutoCompose! We welcome contributions from the community while respecting the terms of our license.

## License Considerations

AutoCompose is licensed under the Olympus Chain Internal Source License (OCISL). Before contributing, please understand:

- ✅ You can read, analyze, and modify the code within this repository
- ✅ You can fork the repository for contribution purposes or personal non-commercial use
- ❌ Commercial use requires explicit written permission from Olympus Chain SAS
- ❌ Redistributing the code in another project is prohibited
- ❌ Integrating the code into third-party software requires written consent

By contributing, you agree that your contributions will be licensed under the same license.

## How to Contribute

### 1. Getting Started

1. **Fork the repository** on GitHub for contribution purposes
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/autocompose.git
   cd autocompose
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/Olympus-chain/autocompose.git
   ```

### 2. Development Setup

**Requirements:**
- Rust 1.65 or later
- Docker Engine 20.10+ or Podman 3.0+
- Git

**Setup:**
```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- docker
```

### 3. Making Changes

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following our coding standards

3. **Test your changes**:
   ```bash
   # Run unit tests
   cargo test

   # Run integration tests
   cargo test --test '*' --features integration

   # Test with Docker
   cargo run -- docker --dry-run

   # Test with Podman
   cargo run -- podman --dry-run
   ```

4. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

### 4. Coding Standards

**Rust Code Style:**
- Follow standard Rust conventions and idioms
- Use `cargo fmt` to format your code
- Use `cargo clippy` to check for common issues
- Add documentation comments for public APIs
- Write unit tests for new functionality

**Commit Messages:**
- Use conventional commit format: `type(scope): description`
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`
- Keep the first line under 72 characters
- Add detailed description if needed

**Examples:**
```
feat(docker): add support for compose v3.9 features
fix(validation): handle empty environment variables correctly
docs(readme): update installation instructions for ARM64
```

### 5. Submitting Pull Requests

1. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create a Pull Request** on GitHub with:
   - Clear title describing the change
   - Detailed description of what and why
   - Reference to any related issues
   - Test results or examples

3. **PR Guidelines:**
   - PRs should focus on a single concern
   - Include tests for new features
   - Update documentation as needed
   - Ensure all tests pass
   - Respond to review feedback promptly

## Priority Contribution Areas

We especially welcome contributions in these areas:

### 🎯 High Priority
- **Container dependency detection**: Algorithms to automatically detect and map container dependencies
- **Performance optimizations**: Improvements to parallel processing and memory usage
- **Security enhancements**: Better secret detection and filtering mechanisms
- **Error handling**: More robust error handling and user-friendly messages

### 🔧 Features
- **Cloud provider integrations**: Support for AWS ECS, Azure Container Instances, Google Cloud Run
- **Additional output formats**: Helm charts, Terraform configurations
- **Network topology visualization**: Generate network diagrams from compose files
- **Compose file merging**: Combine multiple compose files intelligently

### 📚 Documentation
- **Tutorials and guides**: Step-by-step guides for common use cases
- **API documentation**: Detailed documentation of the library API
- **Translations**: Translate documentation to other languages
- **Examples**: Real-world examples and best practices

### 🧪 Testing
- **Integration tests**: More comprehensive integration test coverage
- **Performance benchmarks**: Benchmarking suite for performance tracking
- **Cross-platform testing**: Testing on different OS and container runtime versions

## Code Review Process

1. **Automated checks**: All PRs must pass CI checks (tests, formatting, linting)
2. **Code review**: At least one maintainer will review your code
3. **Feedback**: Address any requested changes
4. **Merge**: Once approved, your PR will be merged

## Reporting Issues

**Before reporting an issue:**
- Check if it's already reported in [existing issues](https://github.com/Olympus-chain/autocompose/issues)
- Try with the latest version
- Collect relevant information (OS, container runtime version, error messages)

**When reporting:**
- Use issue templates if available
- Provide clear reproduction steps
- Include relevant logs and configurations
- Be respectful and constructive

## Development Tips

### Running Specific Tests
```bash
# Run a specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel
cargo test -- --test-threads=4
```

### Debugging
```bash
# Enable debug logging
RUST_LOG=autocompose=debug cargo run -- docker

# Use GDB
rust-gdb target/debug/autocompose

# Profile with valgrind
valgrind --tool=callgrind target/release/autocompose docker
```

### Building for Different Targets
See [docs/CROSS_COMPILATION.md](docs/CROSS_COMPILATION.md) for cross-compilation instructions.

## Community

- **Discussions**: Join our [GitHub Discussions](https://github.com/Olympus-chain/autocompose/discussions)
- **Issues**: Report bugs in [GitHub Issues](https://github.com/Olympus-chain/autocompose/issues)
- **Email**: For commercial inquiries: contact@olympus-chain.fr

## Recognition

Contributors will be recognized in:
- The project's CHANGELOG.md
- GitHub's contributor list
- Special mentions for significant contributions

## Questions?

If you have questions about contributing:
1. Check existing documentation
2. Search closed issues and discussions
3. Ask in GitHub Discussions
4. Contact maintainers if needed

Thank you for helping make AutoCompose better!

---

**Remember**: All contributions must comply with the OCISL license terms. Commercial use or redistribution outside this repository requires explicit permission from Olympus Chain SAS.