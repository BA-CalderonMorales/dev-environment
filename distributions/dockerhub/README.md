# Development Environment Setup

> ⚠️ **IMPORTANT: DockerHub Rate Limits**
> 
> This repository's Docker image (`cmoe640/dev-environment`) is subject to DockerHub's rate limits:
> - Anonymous: 100 pulls/6 hours
> - Authenticated: 200 pulls/6 hours
>
> **Recommended Usage:**
> 1. Fork this repository
> 2. Build and push the image to your own DockerHub account
> 3. Update the docker-compose.yml to use your image
>
> Example:
> ```yaml
> image: your-dockerhub-username/dev-environment:latest
> ```
>
> This ensures your team has dedicated pull limits and control over the environment.

This project provides a consistent development environment using Docker, designed for seamless team collaboration and individual development.

## Current Tool Versions

The environment includes the latest stable versions of:
- Node.js with npm
- Go (secure version)
- Rust with cargo
- Git
- SQLite

Exact versions are displayed in the welcome message when you access the container:

```
╔══════════════════════════════════════════════════╗
║          Development Environment Versions        ║
╠══════════════════════════════════════════════════╣
║ Node.js:  <dynamic-version>                      ║
║ Go:       <dynamic-version>                      ║
║ Rust:     <dynamic-version>                      ║
║ Git:      <dynamic-version>                      ║
║ SQLite:   <dynamic-version>                      ║
╠══════════════════════════════════════════════════╣
║ Working Directory: /usr/src/projects             ║
╚══════════════════════════════════════════════════╝
```

## Quick Start (5 Minutes)

1. Prerequisites:
   * Docker Desktop installed
   * Docker Hub Account (request access to the dev-environment image)

2. Fork First (Recommended):
   ```bash
   # Clone your fork
   git clone https://github.com/YOUR_USERNAME/dev-environment.git
   
   # Build your own image
   cd dev-environment/distributions/dockerhub
   docker build -t YOUR_USERNAME/dev-environment:latest .
   
   # Push to your DockerHub
   docker push YOUR_USERNAME/dev-environment:latest
   ```

3. Or Use Direct Setup (Subject to rate limits):
   ```bash
   mkdir -p C:/dev/projects && cd C:/dev && curl -O https://raw.githubusercontent.com/BA-CalderonMorales/dev-environment/main/distributions/dockerhub/docker-compose.yml && docker compose up -d
   ```

## Why Use This Environment?

- ✨ **Consistent Setup**: Same environment for all team members
- 🚀 **Pre-configured Tools**: Latest stable versions of Node.js, Go, Rust
- 🔄 **Version Control**: Use specific tags for team-wide consistency
- 📦 **Cached Dependencies**: Shared volume mounts for package caches
- 🛠️ **IDE Ready**: VS Code remote development support

## Available Image Tags

- `latest` - Current stable version
- `latest-<commit-hash>` - Specific version tied to a commit

Example using specific version:
```bash
docker pull cmoe640/dev-environment:latest-8325b1a411ad382a64fd6c69ad2f5f50084d2dcc
```

## Initial Setup

1. Create your development directory structure:
   ```bash
   mkdir C:\dev
   cd C:\dev
   mkdir projects
   ```

2. Create a docker-compose.yml file in C:\dev with the following content:
   ```yaml
   version: '3.8'
   services:
     dev:
       image: cmoe640/dev-environment:latest
       container_name: dev-environment
       ports:
         - "8080:8080"    # VS Code
         - "3000-3010:3000-3010"  # Frontend apps
         - "8000-8010:8000-8010"  # Backend services
       volumes:
         - ./projects:/usr/src/projects
         - ~/.ssh:/home/devuser/.ssh:ro
         - vscode-server:/home/devuser/.vscode-server
         - npm-cache:/home/devuser/.npm
         - cargo-cache:/home/devuser/.cargo
         - go-cache:/home/devuser/go
       restart: unless-stopped
       stdin_open: true
       tty: true

   volumes:
     vscode-server:
     npm-cache:
     cargo-cache:
     go-cache:
   ```

## Working with the Environment

### Basic Usage
```bash
# 1. Create local docker-compose.yml file (see github repo)
# 2. Start the environment
docker compose up -d

# 3. Enter the development environment
# For Git Bash:
winpty docker exec -it dev-environment bash
# For other terminals:
docker exec -it dev-environment bash

# 4. Shut down when finished
docker compose down
```

### Development Workflow (For Contributors)
If you plan on iterating on the image or forking the repository:

1. Clean up existing environment:
```bash
# Shut down the environment
docker compose down

# Remove local version of the image
docker rmi cmoe640/dev-environment:latest
```

2. Make your changes to the `Dockerfile` in the `distributions/dockerhub` directory

3. Build and test locally:
```bash
# From distributions/dockerhub directory
docker build -t your-username/dev-environment:latest .
```

4. Update your docker-compose.yml to use your image:
```yaml
image: your-username/dev-environment:latest
```

5. Test your changes:
```bash
docker compose up -d
winpty docker exec -it dev-environment bash
```

## Pro Tips 💡

1. **VS Code Integration**
   ```bash
   # Install Remote Development extension
   code --install-extension ms-vscode-remote.vscode-remote-extensionpack
   ```

2. **Shell Aliases**
   Add to your `.bashrc` or `.zshrc`:
   ```bash
   alias dev='cd C:/dev'
   alias devsh='winpty docker exec -it dev-environment bash'
   ```

3. **Version Management**
   ```bash
   # Pin specific version in docker-compose.yml
   image: cmoe640/dev-environment:latest-8325b1a411ad382a64fd6c69ad2f5f50084d2dcc
   ```

## Image Maintenance

Clean up unused images:
```bash
# Remove unused images
docker image prune

# Remove specific version
docker rmi cmoe640/dev-environment:tag-name

# Remove dangling images
docker image prune --filter="dangling=true"
```

Best Practices:
- Keep `latest` for current stable version
- Maintain last 2-3 versions for potential rollback
- Regularly prune older versions
- Use specific tags for production deployments

## Troubleshooting

1. Rate Limits
   - Ensure you're logged in to Docker Hub
   - Consider forking and using your own image
   - Use specific tags to avoid frequent pulls

2. Common Issues
   - Verify Docker Desktop is running
   - Check port conflicts (8080, 3000-3010, 8000-8010)
   - Ensure proper file permissions in mounted volumes
   - For Git Bash users, remember to use `winpty`

## Need Help?

- 📝 [Report Issues](https://github.com/BA-CalderonMorales/dev-environment/issues)
- 💬 [Team Discussion](https://github.com/BA-CalderonMorales/dev-environment/discussions)
- 📚 [Full Documentation](https://github.com/BA-CalderonMorales/dev-environment/wiki)
- 🔄 [Latest Releases](https://github.com/BA-CalderonMorales/dev-environment/releases)
- 🌟 [Star this repo](https://github.com/BA-CalderonMorales/dev-environment)

## Contributing

Found a bug or want to suggest an improvement? Check out our [contribution guidelines](https://github.com/BA-CalderonMorales/dev-environment/blob/main/CONTRIBUTING.md) or [open an issue](https://github.com/BA-CalderonMorales/dev-environment/issues/new).

## Verifying Installation

After starting the container, you should see this welcome message:
```
╔══════════════════════════════════════════════════╗
║          Development Environment Versions        ║
╠══════════════════════════════════════════════════╣
║ Node.js:  <dynamic-version>                      ║
║ Go:       <dynamic-version>                      ║
║ Rust:     <dynamic-version>                      ║
║ Git:      <dynamic-version>                      ║
║ SQLite:   <dynamic-version>                      ║
╠══════════════════════════════════════════════════╣
║ Working Directory: /usr/src/projects             ║
╚══════════════════════════════════════════════════╝
```
