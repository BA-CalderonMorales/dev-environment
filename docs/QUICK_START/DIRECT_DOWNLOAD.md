# Downloading the Development Environment

## Prerequisites
- Git Bash or similar terminal
- Docker Desktop installed and running

## Method 1: DockerHub (Recommended)
```bash
# Pull the latest stable release
docker pull cmoe640/dev-environment:latest

# Or pull the beta release
docker pull cmoe640/dev-environment:beta

# Run the environment
winpty docker run -it cmoe640/dev-environment:latest
```

## Method 2: Direct Download
```bash
# For latest stable release
curl -L -o dev-environment-latest.tar \
  https://github.com/BA-CalderonMorales/dev-environment/releases/latest/download/dev-environment-latest.tar

# Or for beta release
curl -L -o dev-environment-beta.tar \
  https://github.com/BA-CalderonMorales/dev-environment/releases/download/beta/dev-environment-beta.tar

# Load and run
docker load < dev-environment-latest.tar
winpty docker run -it cmoe640/dev-environment:latest
```

## Troubleshooting
If you encounter any issues:
- Check [Docker Issues](../TROUBLESHOOTING.md#docker-issues)
- Check [Download Issues](../TROUBLESHOOTING.md#download-issues)