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
If you see errors, check the [Troubleshooting Guide](TROUBLESHOOTING.md#docker-issues)

## Option 1: DockerHub (Recommended)
```bash
# Pull and run the environment
docker pull cmoe640/dev-environment:latest
docker run -it cmoe640/dev-environment:latest
```

## Option 2: BitTorrent Download

### Prerequisites
- BitTorrent client (qBittorrent recommended)
- Git Bash or similar terminal

### Steps
1. Get the magnet link:
```bash
# Create directory for downloads
mkdir -p ~/downloads/dev-env && cd ~/downloads/dev-env

# Download magnet link (auto-updated with latest stable release)
curl -L -o magnet.txt \
  https://github.com/BA-CalderonMorales/dev-environment/releases/latest/download/magnet.txt

# Verify downloaded file
cat magnet.txt
```

2. Start the download:
   - Open your BitTorrent client
   - Import the magnet link from `magnet.txt`
   - Wait for download to complete

3. Load and run:
```bash
# Load the Docker image
docker load < dev-environment-latest.tar

# Run the environment
docker run -it cmoe640/dev-environment:latest
```

> **Note**: The download links are automatically updated by our CI/CD pipeline 
> to point to the latest stable release.

## Option 3: Setup Script
```bash
# Download and run setup script
curl -fsSL https://raw.githubusercontent.com/yourusername/dev-environment/main/startup/start-dev.sh | bash
```

## Option 4: Manual Setup
1. Clone the repository:
```bash
git clone https://github.com/yourusername/dev-environment.git
cd dev-environment
```

2. Choose distribution method:
   - DockerHub (faster, requires good internet)
   - BitTorrent (better for slow connections)

## Shutting Down

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
1. Check the [Usage Guide](USAGE_GUIDE.md) for detailed instructions
2. Explore available development tools
3. Review [Troubleshooting](TROUBLESHOOTING.md) if needed