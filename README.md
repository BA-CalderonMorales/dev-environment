# Development Environment Setup

This repository provides multiple distribution methods for a consistent development environment.

## Available Distributions

### 1. DockerHub Distribution (Standard)
- Traditional Docker workflow
- Rate limits apply
- Simple setup process
- [DockerHub Distribution Documentation](distributions/dockerhub/README.md)

### 2. BitTorrent Distribution (Experimental)
- Decentralized distribution
- No rate limits
- [BitTorrent Distribution Documentation](distributions/bittorrent/README.md)

## Project Structure
```
repository/
├── distributions/        # Different distribution methods
│   ├── dockerhub/       # Standard DockerHub distribution
│   └── bittorrent/      # BitTorrent-based distribution
├── startup/             # Setup and initialization scripts
│   ├── lib/            # Shared library functions
│   ├── templates/      # Project templates
│   ├── init-project.sh # Project initialization script
│   └── start-dev.sh    # Environment startup script
├── projects/           # Your development workspace
└── .github/            # GitHub Actions and workflows
```

## Quick Start
Choose a distribution method and follow its specific documentation.

## Contributing
We welcome contributions for new distribution methods! See our [contribution guidelines](CONTRIBUTING.md).