# Troubleshooting Guide

## Quick Reference

💡 Most issues can be solved by:
1. Open Docker Desktop
2. Wait 1-2 minutes for the whale icon to stop animating
3. Try your docker command again

## Installation & Setup

### Docker Tags
Choose the right tag for your needs:
- `:latest` - Stable, production-ready (recommended)
- `:beta` - Release candidate, feature-complete but under testing
- `:dev` - Latest features, potentially unstable
- `:pipeline` - CI/CD builds (not for general use)

### Common Installation Issues

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

### Permission Issues
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

## Platform-Specific Issues

### Windows Issues

#### Git Bash: winpty Error
When running interactive containers in Git Bash, you might see terminal errors.

**Solution:**
Add `winpty` before your docker run commands:
```bash
winpty docker run -it cmoe640/dev-environment:latest
```

### Docker Desktop Connection Error
```bash
error: open //./pipe/dockerDesktopLinuxEngine: The system cannot find the file specified
```

**Solution:**
1. Check Docker Desktop is running (whale icon in system tray)
2. Wait 1-2 minutes for full initialization
3. Try command again

### Docker Connection Issues
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

### Resource Issues
#### Resource Constraints
```bash
Error: Insufficient memory/CPU
```
**Solution:**
1. Open Docker Desktop settings
2. Increase memory/CPU allocation
3. Restart Docker Desktop

### Distribution Issues

#### Download Issues
```bash
Error: Could not resolve host
```
**Solution:**
1. Check your internet connection
2. Try alternate download command:
```bash
wget https://github.com/BA-CalderonMorales/dev-environment/releases/latest/download/dev-environment-latest.tar
```

#### Docker Load Issues
```bash
Error: invalid tar header
```
**Solution:**
1. Delete partial download
2. Re-download the file
3. Verify the download completed:
```bash
ls -lh dev-environment-latest.tar
```

## Development Issues

### E2E Test Failures
If your E2E tests are failing, verify:
1. Docker daemon is running
2. Correct tag is being used
3. Required permissions are set
4. No network conflicts

### Log Access
Logs are stored in `startup/logs/` inside the container.

### Security Issues
Report security concerns by:
1. Opening a draft security advisory
2. Submitting a security-related PR
3. Issues are handled sequentially (first come, first serve)

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
2. Create a new issue with:
   - Error message
   - Steps to reproduce
   - System information
   - Environment details

## Contributing

Want to improve this guide?
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a PR to `develop`

Include:
- Clear problem description
- Detailed solution steps
- Relevant examples
- Links to related documentation