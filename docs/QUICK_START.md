# Quick Start Guide

## Prerequisites
- Docker ([Install Guide](https://docs.docker.com/get-docker/))
- Git (for manual setup)
- VS Code (recommended)

## Option 1: DockerHub (Recommended)
```bash
# Pull and run the environment
docker pull cmoe640/dev-environment:latest
docker run -it cmoe640/dev-environment:latest
```

## Option 2: Setup Script
```bash
# Download and run setup script
curl -fsSL https://raw.githubusercontent.com/yourusername/dev-environment/main/startup/start-dev.sh | bash
```

## Option 3: Manual Setup
1. Clone the repository:
```bash
git clone https://github.com/yourusername/dev-environment.git
cd dev-environment
```

2. Choose distribution method:
   - DockerHub (faster, requires good internet)
   - BitTorrent (better for slow connections)

## Next Steps
1. Check the [Usage Guide](USAGE_GUIDE.md) for detailed instructions
2. Explore available development tools
3. Review [Troubleshooting](TROUBLESHOOTING.md) if needed 