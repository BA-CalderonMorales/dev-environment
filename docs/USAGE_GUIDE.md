# Usage Guide

## Overview
This development environment provides a consistent, tested workspace using either DockerHub or direct download distribution methods.

## Getting Started

### Prerequisites
- Docker Engine v20.10.0 or higher
- Git

### Installation Methods

#### Method 1: DockerHub (Recommended)
The fastest way to get started is using our DockerHub distribution:

```bash
docker pull cmoe640/dev-environment:latest
docker run -it cmoe640/dev-environment
```

#### Method 2: Direct Download
For situations where DockerHub access is limited:

1. Download the package from our release page
2. Verify the checksum
3. Run the installation script

```bash
# Verify package
sha256sum dev-environment.tar.gz

# Install
./install.sh
```

## Using the Environment

### Starting the Environment
```bash
# Start with default settings
./startup/start-dev.sh
```

### Development Workflow
1. Start the environment
2. Access your workspace
3. Use integrated development tools
4. Commit and push changes

### Common Tasks
- Access container shell: `docker exec -it dev-environment bash`
- Update environment: `./startup/update.sh`
- Check status: `./startup/status.sh`

## Next Steps
- Read the [Contributing Guide](CONTRIBUTING.md) to help improve the project
- Check [Troubleshooting](TROUBLESHOOTING.md) for common issues
- Explore [E2E Tests](E2E_TESTS.md) for testing details