# DockerHub Distribution

This distribution method uses DockerHub to provide a consistent development environment.

## Quick Start (5 Minutes)

1. **Prerequisites**:
   * Docker Desktop installed
   * Internet connection (for pulling from DockerHub)

2. **Start the Environment**:
   ```bash
   # From the repository root
   ./startup/start-dev.sh
   ```

## Development Workflow

1. **Clean Existing Environment**:
   ```bash
   # Shut down any running environment
   docker compose down

   # Remove local version of the image
   docker rmi cmoe640/dev-environment:latest
   ```

2. **Build Locally** (for development):
   ```bash
   # From distributions/dockerhub directory
   docker build -t dev-environment:latest .
   ```

3. **Test Your Changes**:
   ```bash
   # Start the environment
   docker compose up -d

   # Access the container
   docker exec -it dev-environment bash
   ```

## Troubleshooting

1. **Rate Limits**:
   - If you encounter rate limits, the system will automatically try the BitTorrent distribution
   - You can also use:
     ```bash
     export PREFER_BITTORRENT=true
     ./startup/start-dev.sh
     ```

2. **Version Issues**:
   - Pin to a specific version in docker-compose.yml:
     ```yaml
     image: cmoe640/dev-environment:latest-<commit-hash>
     ```
   - Check available versions on [DockerHub](https://hub.docker.com/r/cmoe640/dev-environment/tags)

## Pro Tips 💡

1. **VS Code Integration**:
   ```bash
   # Install Remote Development extension
   code --install-extension ms-vscode-remote.vscode-remote-extensionpack
   ```

2. **Shell Aliases**:
   ```bash
   # Add to your .bashrc or .zshrc
   alias dev='cd path/to/workspace'
   alias devsh='docker exec -it dev-environment bash'
   ```

## E2E Testing
Run the test suite to verify your changes:
```bash
# From the repository root
./distributions/dockerhub/e2e/scripts/test.sh
```
