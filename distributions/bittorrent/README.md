# BitTorrent Distribution

This distribution method uses BitTorrent to distribute the development environment image, bypassing traditional rate limits.

## Quick Start
```bash
# From the repository root
./startup/start-dev.sh --prefer-bittorrent
```

## Prerequisites
- transmission-cli (installed automatically if missing)
- Docker and Docker Compose

## How It Works
1. Downloads the latest magnet link
2. Uses transmission-cli to download the image
3. Loads the image into Docker
4. Starts the environment

## Development

If you plan on iterating on the image or forking the repository:

1. Clean up existing environment:
```bash
# Shut down the environment
docker compose down

# Remove local version of the image
docker rmi dev-environment:latest
```

2. Make your changes to the `Dockerfile`

3. Build and test locally:
```bash
# From distributions/bittorrent directory
docker build -t dev-environment:latest .
```

4. Test your changes:
```bash
docker compose up -d
docker exec -it dev-environment bash
```

## Directory Structure
```
bittorrent/
├── scripts/
│   ├── build/          # Image building scripts
│   ├── torrent/        # Torrent creation and seeding
│   └── seeding/        # Persistent seeding setup
├── e2e/               # End-to-end tests
└── docker-compose.yml # Container configuration
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
   alias dev='cd path/to/workspace'
   alias devsh='docker exec -it dev-environment bash'
   ```

3. **Troubleshooting**
   ```bash
   # Check torrent download status
   transmission-remote -l
   
   # Force DockerHub fallback
   export FORCE_BITTORRENT_FAIL=true
   ./startup/start-dev.sh
   ```

## ⚠️ Important Notes
- This is an experimental distribution method
- Availability depends on peer seeding
- Consider building locally for production use
