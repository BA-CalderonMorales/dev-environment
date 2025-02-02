# Usage Guide

## Quick Start

### DockerHub Installation
```bash
# Pull stable version
docker pull cmoe640/dev-environment:latest

# Or development version
docker pull cmoe640/dev-environment:dev
```

### Tag Selection
- `:latest` - Stable release (recommended)
- `:beta` - Pre-release testing
- `:dev` - Latest features
- `:pipeline` - CI/CD builds (avoid)

## Basic Usage

### Starting the Environment
```bash
docker run -it cmoe640/dev-environment:latest
```

### Development Workflow
1. Pull latest version
2. Start container
3. Begin development
4. Commit changes
5. Push to repository

## Advanced Usage

### Custom Configuration
```bash
# Mount custom config
docker run -v ~/.myconfig:/config -it cmoe640/dev-environment:latest
```

### Troubleshooting
See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common issues and solutions.

## Next Steps
- Read the [Contributing Guide](CONTRIBUTING.md) to help improve the project
- Check [Troubleshooting](TROUBLESHOOTING.md) for common issues
- Explore [E2E Tests](E2E_TESTS.md) for testing details