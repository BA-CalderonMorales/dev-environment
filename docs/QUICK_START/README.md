# Quick Start Guide

## Prerequisites
- Docker Desktop ([Install Guide](https://docs.docker.com/get-docker/))
- Git (for manual setup)
- VS Code (recommended)

## Before Starting
1. Ensure Docker Desktop is running:
   - Look for the Docker Desktop icon in your system tray
   - On Windows, open Task Manager and verify "Docker Desktop" is running
   - If not running, launch Docker Desktop and wait for it to fully start

2. Verify Docker is working:
```bash
docker --version
docker ps
```
If you see errors, check the [Troubleshooting Guide](../TROUBLESHOOTING.md#docker-issues)

## Distribution Options
1. [DockerHub](DOCKERHUB.md) (Recommended, faster with good internet)
2. [BitTorrent](BITTORRENT.md) (Better for slow connections)

## Common Setup Steps

### Workspace Setup
```bash
# Create and enter your project directory (name it whatever you prefer)
mkdir -p ~/my-dev-environment && cd ~/my-dev-environment
```

## Managing Your Environment

### Temporary Stop (Resume Later)
```bash
# From another terminal, find the container ID
docker ps

# Stop the container
docker stop <container-id>

# To resume later
docker start -i <container-id>
```

### Complete Cleanup
```bash
# Stop and remove container
docker stop <container-id>
docker rm <container-id>

# Optionally remove the image
docker rmi cmoe640/dev-environment:latest
```

## Next Steps
1. Check the [Usage Guide](../USAGE_GUIDE.md) for detailed instructions
2. Explore available development tools
3. Review [Troubleshooting](../TROUBLESHOOTING.md) if needed
