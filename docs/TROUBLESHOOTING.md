# Troubleshooting Guide

## Common Issues

### Docker Issues

#### Docker Not Installed
```bash
Error: Cannot connect to the Docker daemon
```
**Solution:**
1. [Install Docker](https://docs.docker.com/get-docker/)
2. Ensure Docker service is running:
```bash
# Linux
sudo systemctl start docker

# macOS/Windows
# Start Docker Desktop
```

#### Permission Issues
```bash
Error: Got permission denied while trying to connect to the Docker daemon socket
```
**Solution:**
```bash
# Add your user to the docker group
sudo usermod -aG docker $USER

# Then log out and back in
```

### Environment Issues

#### Container Won't Start
```bash
Error: Unable to find image 'cmoe640/dev-environment:latest' locally
```
**Solution:**
1. Check your internet connection
2. Try pulling explicitly:
```bash
docker pull cmoe640/dev-environment:latest
```

#### Resource Constraints
```bash
Error: Insufficient memory/CPU
```
**Solution:**
1. Open Docker Desktop settings
2. Increase memory/CPU allocation
3. Restart Docker Desktop

### Distribution Issues

#### DockerHub Rate Limit
```bash
Error: You have reached your pull rate limit
```
**Solution:**
1. Wait for rate limit reset, or
2. Use BitTorrent distribution instead:
   - Download from [latest release](https://github.com/yourusername/dev-environment/releases/latest)

#### BitTorrent Issues
```bash
Error: Checksum verification failed
```
**Solution:**
1. Delete partial download
2. Re-download torrent file
3. Verify using:
```bash
sha256sum -c checksum.txt
```

### Setup Script Issues

#### Script Permission Denied
```bash
Error: Permission denied: ./setup.sh
```
**Solution:**
```bash
chmod +x setup.sh
./setup.sh
```

#### Script Download Failed
```bash
Error: Failed to download setup script
```
**Solution:**
1. Check your internet connection
2. Try manual download:
```bash
wget https://raw.githubusercontent.com/yourusername/dev-environment/main/startup/start-dev.sh
```

## Getting Help

If your issue isn't covered here:
1. Check [existing issues](https://github.com/yourusername/dev-environment/issues)
2. Check our [FAQ](FAQ.md)
3. Create a new issue with:
   - Error message
   - Steps to reproduce
   - System information 