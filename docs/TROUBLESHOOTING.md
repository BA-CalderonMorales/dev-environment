# Troubleshooting Guide

## Quick Fixes for Common Issues

💡 Most issues can be solved by:
1. Open Docker Desktop
2. Wait 1-2 minutes for the whale icon to stop animating
3. Try your docker command again

## Windows-Specific Issues

### Git Bash: winpty Error
When running interactive containers in Git Bash, you might see terminal errors.

**Solution:**
Add `winpty` before your docker run commands:
```bash
winpty docker run -it cmoe640/dev-environment:latest
```

Or create an alias in your ~/.bashrc:
```bash
alias docker="winpty docker"
```

### Docker Desktop Connection Error
```bash
error: open //./pipe/dockerDesktopLinuxEngine: The system cannot find the file specified
```

**Solution:**
1. Check Docker Desktop is running (whale icon in system tray)
2. Wait 1-2 minutes for full initialization
3. Try command again

## Docker Issues by Category

### 1. Installation Issues
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

### 2. Permission Issues
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

### 3. Connection Issues
#### "Cannot find the file specified" or Docker Connection Errors

Follow these steps in order:

1. Check if Docker Desktop is Running
   - Look for the whale icon in your system tray
   - If you don't see it, open Docker Desktop from your Start Menu

2. Wait for Docker to Start
   - Docker Desktop needs 1-2 minutes to fully initialize
   - Watch for the whale icon to stop animating
   - The icon should turn solid when ready

3. Verify Docker Works
   ```bash
   docker version
   ```
   If this works, proceed to pull the image again:
   ```bash
   docker pull cmoe640/dev-environment:latest
   ```

4. If Still Not Working
   - Right-click the whale icon
   - Select "Restart"
   - Wait 2 minutes
   - Try the docker pull command again

5. Last Resort
   - Quit Docker Desktop completely
   - Restart your computer
   - Start Docker Desktop
   - Wait 2 minutes before trying again

Need more help? Visit our [GitHub Issues](https://github.com/yourusername/dev-environment/issues)

### 4. Resource Issues
#### Resource Constraints
```bash
Error: Insufficient memory/CPU
```
**Solution:**
1. Open Docker Desktop settings
2. Increase memory/CPU allocation
3. Restart Docker Desktop

### 5. Distribution Issues
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

## Terminal-Specific Issues

### Git Bash
- Use `winpty` for interactive containers
- Use forward slashes for paths
- Use proper line endings (LF)

### PowerShell
// ...existing code...

### CMD
// ...existing code...

## Getting Help

If your issue isn't covered here:
1. Check [existing issues](https://github.com/yourusername/dev-environment/issues)
2. Check our [FAQ](FAQ.md)
3. Create a new issue with:
   - Error message
   - Steps to reproduce
   - System information