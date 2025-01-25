# Usage Guide

## Overview
This development environment provides a consistent, tested workspace using either DockerHub or BitTorrent distribution methods.

## Getting Started

### Prerequisites
- Docker Engine v20.10.0 or higher
- Git
- (Optional) BitTorrent client for alternative distribution

### Installation Methods

#### 1. DockerHub (Recommended)
```bash
# Pull the latest release
docker pull cmoe640/dev-environment:latest

# Or pull a specific version
docker pull cmoe640/dev-environment:v0.1.0
```

#### 2. BitTorrent Distribution
1. Download the .torrent file from the latest release
2. Use your preferred torrent client
3. Verify the checksum:
```bash
sha256sum -c checksum.txt
```

## Using the Environment

### Starting the Environment
```bash
# Start with default settings
./startup/start-dev.sh

# Start with BitTorrent distribution
PREFER_BITTORRENT=true ./startup/start-dev.sh
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