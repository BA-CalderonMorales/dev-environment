# Contributing

For detailed contributor and maintainer documentation, please see [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md).

## Adding New Distributions

1. Create a new directory under `distributions/`
2. Implement one of the approved distribution methods:
   - DockerHub distribution
   - Direct download distribution
3. Include required files:
   - README.md (distribution-specific documentation)
   - docker-compose.yml (for container-based distributions)
   - Distribution-specific configuration
   - Security verification mechanisms (checksums/signatures)

## Distribution Requirements
- Clear documentation
- Setup instructions
- Configuration details
- Usage examples
- E2E test suite
- Fallback mechanisms
- Version tracking