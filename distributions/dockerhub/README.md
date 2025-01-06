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
         - ~/.gitconfig:/home/devuser/.gitconfig:ro
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

## Development Workflow

### 1. Daily Startup
```bash
# Start your day
cd C:/dev
docker compose up -d

# Access container (Git Bash)
winpty docker exec -it dev-environment bash

# Access container (CMD/PowerShell)
docker exec -it dev-environment bash
```

### 2. Project Structure
```
C:/dev/                    Container:
├── projects/             /usr/src/projects/
│   ├── project1/         ├── project1/
│   └── project2/         └── project2/
└── docker-compose.yml
```

### 3. Common Tasks

#### Start New Project
```bash
cd /usr/src/projects
mkdir my-new-project && cd my-new-project

# Node.js Project
npm init -y

# Go Project
go mod init myproject

# Rust Project
cargo init
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
║          Development Environment Versions         ║
╠══════════════════════════════════════════════════╣
║ Node.js:  <dynamic-version>                      ║
║ Go:       <dynamic-version>                      ║
║ Rust:     <dynamic-version>                      ║
║ Git:      <dynamic-version>                      ║
║ SQLite:   <dynamic-version>                      ║
╠══════════════════════════════════════════════════╣
║ Working Directory: /usr/src/projects            ║
╚══════════════════════════════════════════════════╝
```
