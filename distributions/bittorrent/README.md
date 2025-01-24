# BitTorrent Distribution

This distribution method uses BitTorrent to distribute the development environment image, bypassing traditional rate limits.

## Quick Start (5-10 Minutes)

1. **Prerequisites**:
   * Docker Desktop installed
   * Internet connection (for torrent download)
   * transmission-cli (will be auto-installed if missing)

2. **Start the Environment**:
   ```bash
   # From the repository root
   ./startup/start-dev.sh --prefer-bittorrent
   ```

## How It Works
1. Downloads the latest magnet link
2. Uses transmission-cli to download the image
3. Loads the image into Docker
4. Starts the environment

## Development Workflow

1. **Clean Existing Environment**:
   ```bash
   # Shut down any running environment
   docker compose down

   # Remove local version of the image
   docker rmi dev-environment:latest
   ```

2. **Build Locally** (for development):
   ```bash
   # From distributions/bittorrent directory
   docker build -t dev-environment:latest .
   ```

3. **Test Your Changes**:
   ```bash
   # Start the environment
   docker compose up -d

   # Access the container
   docker exec -it dev-environment bash
   ```

## Troubleshooting

1. **Download Issues**:
   - If torrent download fails, the system will automatically fall back to DockerHub
   - Check torrent status:
     ```bash
     transmission-remote -l
     ```
   - Force DockerHub fallback:
     ```bash
     export FORCE_BITTORRENT_FAIL=true
     ./startup/start-dev.sh
     ```

2. **Slow Downloads**:
   - Check your connection
   - Verify peer availability
   - Consider using DockerHub distribution instead

## Pro Tips 💡

1. **VS Code Integration**:
   ```bash
   # Install Remote Development extension
   code --install-extension ms-vscode-remote.vscode-remote-extensionpack
   ```

2. **Shell Aliases**:
   ```bash
   # Add to your .bashrc or .zshrc
   alias dev='cd path/to/workspace'
   alias devsh='docker exec -it dev-environment bash'
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

## Testing
For end-to-end testing instructions, see [E2E Testing Documentation](../../docs/E2E_TESTS.md).
