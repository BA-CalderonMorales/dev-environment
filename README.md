# Development Environment Setup

This repository provides multiple distribution methods for a consistent development environment.

## ⚠️ Important Notice
All distributions are subject to availability constraints:
- DockerHub: Rate limits apply
- BitTorrent: Peer availability dependent
- Consider building locally for production use

## Latest Release
The latest release includes:
- Verified builds from all distribution methods
- E2E test results
- Full documentation
- Usage warnings and disclaimers

[View Latest Release](../../releases/latest)

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