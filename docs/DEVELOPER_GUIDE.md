# Developer Guide

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

## Contributing

### Setting Up Development Environment
1. Clone the repository:
```bash
git clone https://github.com/yourusername/dev-environment.git
cd dev-environment
```

2. Install Rust toolchain:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

### Running Tests
```bash
# Run all tests
cd e2e
cargo run --release -- creator \
  --dockerfile "../distributions/dockerhub/Dockerfile" \
  --dockerhub-repo "your-repo/dev-environment"
```

### Making Changes
1. Create a feature branch
2. Make your changes
3. Run the test suite
4. Submit a pull request

### Release Process
- Successful tests on main/develop trigger automatic releases
- Versions follow semantic versioning (MAJOR.MINOR.PATCH)
- Each release includes:
  - Docker image
  - BitTorrent distribution
  - Checksums for verification 