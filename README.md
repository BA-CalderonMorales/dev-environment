# Development Environment Setup

This project provides a consistent and easy-to-use development environment based on Docker.

## Prerequisites

* Docker Desktop installed
* Docker Hub Account (request access to the dev-environment image)

## Setup Steps

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

3. Generate a Docker Hub Access Token:
   - Go to Docker Hub → Account Settings → Security → New Access Token
   - Create a token with read-only permissions
   - Save the token somewhere secure - you'll need it for the next step

4. Log in to Docker Hub (choose one method):
   ```bash
   # Method 1: Interactive login (if you have a TTY terminal)
   docker login
   
   # Method 2: Direct login with credentials
   docker login -u your-username -p your-access-token
   
   # Method 3: Secure login using password-stdin
   echo "your-access-token" | docker login -u your-username --password-stdin
   ```
   Note: Replace 'your-username' with your Docker Hub username and 'your-access-token' with your Docker Hub access token

5. Start the development environment:
   ```bash
   cd C:\dev
   docker compose up -d
   ```

6. Access the container:
   ```bash
   docker exec -it dev-environment bash
   ```

## Working with Projects

All your projects should be created within the `C:\dev\projects` directory. This directory is mounted inside the container at `/usr/src/projects`.

## Available Development Tools

The development environment comes with:
- Node.js v22.3.0
- Go v1.21.7
- Rust (latest stable)
- Git
- SQLite3

## Stopping the Environment

To stop the development environment:
```bash
cd C:\dev
docker compose down
```

## Troubleshooting

If you encounter permission issues or access denied errors:
1. Ensure you're logged in to Docker Hub
2. Verify your access token hasn't expired
3. Check that Docker Desktop is running
4. Ensure all paths in docker-compose.yml exist on your system
