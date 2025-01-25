# BitTorrent Distribution

Better option for slow or unreliable internet connections.

## Prerequisites
- BitTorrent client (qBittorrent recommended)
- Git Bash or similar terminal

## Steps
1. Get the magnet link:
```bash
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

## Troubleshooting
If you encounter any issues:
- Check [BitTorrent Issues](../TROUBLESHOOTING.md#bittorrent-issues)
- View [BitTorrent Download Issues](../TROUBLESHOOTING.md#bittorrent-download-issues)
