# Discrepancies Between French Documentation and CLI Implementation

## Summary

This document provides a comprehensive analysis of discrepancies between the French documentation (`docs/v1.5/fr/README.md`) and the actual CLI implementation in AutoCompose v1.5.0.

## 1. Missing Command-Line Options (Documented but Not Implemented)

### Basic Options Not Implemented:
- `--filter` / `-f` (documented on line 89, 284) - Only `--filter-name` and `--filter-image` exist
- `--exclude` (documented on line 95, 293) - Only `--exclude-name` exists
- `--exclude-system` (documented on line 95) - Implemented as `--include-system` with opposite logic
- `--preview` (documented on line 107) - Does not exist
- `--compact` (documented on line 149) - No compact output mode implemented
- `--no-filters` (documented on line 494) - Does not exist
- `--debug` / `-vvv` (documented on lines 503, 507) - No debug mode implemented
- `debug-info` command (documented on line 513) - Not implemented

### Docker-Specific Options Not Implemented:
- `--docker-host` (documented on line 205) - Not implemented
- `--context` (documented on line 209) - Not implemented
- `--include-labels` (documented on line 212) - Not implemented
- `--preserve-ids` (documented on line 215) - Not implemented
- `--include-networks` (documented on line 223) - Not implemented
- `--preserve-aliases` (documented on line 227) - Not implemented
- `--network-details` (documented on line 230) - Not implemented
- `--include-volumes` (documented on line 237) - Not implemented
- `--convert-mounts` (documented on line 240) - Not implemented
- `--volume-details` (documented on line 243) - Not implemented
- `--network` (documented on line 181) - Not implemented
- `--remove-caps` (documented on lines 193, 687) - Not implemented
- `--docker-socket` (documented on line 470) - Not implemented

### Podman-Specific Options Not Implemented:
- `--systemd-compatible` (documented on line 259) - Not implemented
- `--preserve-selinux` (documented on line 262) - Not implemented
- `--pod` (documented on line 269) - Not implemented
- `--group-by-pod` (documented on line 272) - Not implemented
- `--include-infra` (documented on line 275) - Not implemented

### Filtering Options Not Implemented:
- `--filter-regex` (documented on line 287) - Not implemented
- Multiple `--filter` options (documented on line 290) - Cannot specify multiple patterns
- Multiple `--exclude` options (documented on line 293) - Cannot specify multiple patterns
- `--health` (documented on line 309) - Not implemented
- `--label-regex` (documented on line 325) - Not implemented

### Output Format Options Not Implemented:
- `--compose-version` short form `-v` (documented on lines 163, 398, 524) - Only long form exists
- TOML format mentioned in docs but not clearly marked as available

## 2. Command Structure Differences

### Major Structural Difference:
The documentation shows a flat command structure:
```bash
autocompose [OPTIONS] [CONTAINERS...]
```

But the actual implementation uses subcommands:
```bash
autocompose docker [OPTIONS] [CONTAINERS...]
autocompose podman [OPTIONS] [CONTAINERS...]
autocompose config [SUBCOMMAND]
autocompose validate [OPTIONS] FILE
```

This is a fundamental difference that affects all usage examples in the documentation.

## 3. Incorrectly Documented Options

### Version Flag:
- Documentation shows `--version` / `-V` (line 528)
- Implementation likely uses standard clap version handling

### Output Option:
- Documentation shows default as `docker-compose.yml` (line 159)
- Implementation confirms this is correct

### Format Option:
- Documentation shows `-f` as short form for format (line 524)
- Implementation has no short form for `--format`

### Running Only:
- Documentation shows `--running-only` as including only running containers
- Implementation has both `--running-only` and `--all` with inverse logic

## 4. Missing Features

### Configuration File:
- Documentation mentions `~/.autocompose/config.yml` (line 338)
- Implementation uses `~/.config/autocompose/config.toml` (TOML format, not YAML)

### Environment Variables Not Implemented:
- `AUTOCOMPOSE_CONFIG` (line 375)
- `DOCKER_HOST` (line 378) - Standard Docker env var, but not specifically handled
- `AUTOCOMPOSE_OUTPUT_DIR` (line 381)
- `AUTOCOMPOSE_LOG_LEVEL` (line 384)

### Validation Options Not Fully Implemented:
- `--security` validation flag (documented on line 404) - Not implemented
- Specific validation version with `-v` short form (line 398) - Only long form exists

## 5. Undocumented But Implemented Options

### Docker Command Options:
- `--filter-name` - Filter by container name pattern
- `--filter-image` - Filter by image pattern
- `--exclude-name` - Exclude by name pattern
- `--separate-networks` - Generate separate network definitions
- `--separate-volumes` - Generate separate volume definitions
- `--include-sensitive` - Include sensitive environment variables

### Podman Command Options:
- All the same options as Docker, plus:
- `--include-pods` - Include pods (mentioned in docs but with different flag)
- `--podman-rootless` - Use rootless mode

### Config Command:
The entire config subcommand structure is different:
- `config show` - Show current configuration
- `config set KEY VALUE` - Set configuration value
- `config reset` - Reset to defaults
- `config init [--force]` - Initialize config file

### Validate Command Options:
- `--check-best-practices` - Implemented
- `--compose-version` - Without short form
- `--format` - Output format for validation
- `--strict` - Strict validation mode

## 6. Feature Parity Issues

### Between Docker and Podman:
- Docker has `--running-only` option
- Podman lacks `--running-only` option (missing from PodmanArgs)
- Otherwise, most options are consistent between the two

### Interactive Mode:
- Implemented for both Docker and Podman
- Works differently than documented (uses container selection dialog)

## 7. Configuration Format Differences

### File Format:
- Documentation shows YAML configuration (line 344-369)
- Implementation uses TOML format

### Configuration Keys:
- Documentation shows nested YAML structure
- Implementation uses dot notation for nested keys (e.g., `filters.exclude_system_containers`)

## 8. Required Implementation Work

To match the documentation, the following options need to be implemented:

### High Priority (Core Functionality):
1. Flat command structure (no subcommands) or update documentation
2. `--filter` with wildcard/regex support
3. `--exclude` with multiple patterns
4. `--exclude-system` flag
5. `--docker-host` and `--context` for remote Docker
6. `--include-networks` and `--include-volumes`
7. Debug/verbose output modes

### Medium Priority (Enhanced Features):
1. `--preview` option (though `--dry-run` exists)
2. `--compact` output mode
3. `--remove-caps` for security
4. Network and volume detail options
5. Podman-specific systemd and SELinux options
6. Health and state filtering options

### Low Priority (Nice to Have):
1. `--preserve-ids` and `--preserve-aliases`
2. `--convert-mounts` option
3. `debug-info` command
4. Environment variable support
5. Short forms for various options

## Recommendations

1. **Update Documentation**: The documentation should be updated to reflect the actual subcommand structure
2. **Implement Missing Core Features**: Focus on implementing the most commonly used options first
3. **Standardize Option Names**: Ensure consistency between what's documented and implemented
4. **Add Debug Support**: Implement verbose/debug output for troubleshooting
5. **Configuration Format**: Either change implementation to YAML or update docs to show TOML