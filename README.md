# Development Environment

A consistent, pre-configured development environment that just works.

## Getting Started
- [Quick Start Guide](docs/QUICK_START/README.md) - Get up and running in minutes
  - [DockerHub Setup](docs/QUICK_START/DOCKERHUB.md) - Fastest method
  - [Direct Download](docs/QUICK_START/DOWNLOAD.md) - Alternative method
- [Requirements](docs/REQUIREMENTS.md) - System requirements and prerequisites
- [Features](docs/FEATURES.md) - Full list of included tools and capabilities

## Installation

### Available Tags
We maintain several Docker image tags for different use cases:

- `:latest` - Production-ready, stable release (recommended)
- `:beta` - Release candidate, feature-complete but under testing
- `:dev` - Development branch builds with latest features
- `:pipeline` - CI/CD builds (not recommended for general use)

Choose your preferred installation method:

1. **DockerHub** (Recommended): 
   ```bash
   # For stable release (recommended)
   docker pull cmoe640/dev-environment:latest

   # For beta features
   docker pull cmoe640/dev-environment:beta

   # For development version
   docker pull cmoe640/dev-environment:dev
   ```

2. **Direct Download**: Visit our [latest release](https://github.com/BA-CalderonMorales/dev-environment/releases/latest) page.

See our [download guide](docs/QUICK_START/DOWNLOAD.md) for detailed instructions.

## Documentation
- [Usage Guide](docs/USAGE_GUIDE.md) - Detailed usage instructions
- [Troubleshooting](docs/TROUBLESHOOTING.md) - Common issues and solutions
- [FAQ](docs/FAQ.md) - Frequently asked questions

## Contributing

- [Contributing Guide](docs/CONTRIBUTING.md) - How to contribute to this project

- [Code of Conduct](CODE_OF_CONDUCT.md) - Community guidelines