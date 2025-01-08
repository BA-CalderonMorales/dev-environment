# BitTorrent Distribution

This distribution method uses BitTorrent to distribute the development environment image, bypassing traditional rate limits.

## Quick Start
```bash
# From the repository root
./startup/start-dev.sh --prefer-bittorrent
```

## Prerequisites
- **transmission-cli**: This will be installed automatically if missing.
- **Docker** and **Docker Compose**: Ensure these are installed on your system.

## How It Works
1. **Download the Image**: The script downloads the latest magnet link and uses `transmission-cli` to download the image.
2. **Load the Image into Docker**: After downloading, the image is loaded into Docker using `docker load`.
3. **Start the Environment**: The script then starts the Docker environment using `docker compose up -d`.

## Detailed Steps

1. **Install Prerequisites**:
   - Ensure `transmission-cli` is installed. You can check this by running:
     ```bash
     transmission-cli --version
     ```
   - Ensure Docker and Docker Compose are installed:
     ```bash
     docker --version
     docker-compose --version
     ```

2. **Run the Download Script**:
   - Execute the following command to start the download and setup:
     ```bash
     ./startup/start-dev.sh --prefer-bittorrent
     ```

3. **Verify the Download**:
   - Check the output of the script to ensure that the image was downloaded successfully.
   - If the download fails, ensure you have a stable internet connection and sufficient peers for the torrent.

4. **Load the Image**:
   - The script will automatically load the image into Docker. You can verify this by running:
     ```bash
     docker images
     ```
   - Look for `dev-environment:latest` in the list.

5. **Start the Environment**:
   - The script will start the Docker environment. You can check the status of the containers with:
     ```bash
     docker ps
     ```

6. **Verify the Setup**:
   - Once the environment is running, you can access the services as defined in the `docker-compose.yml`.

## Development

If you plan on iterating on the image or forking the repository:

1. Clean up existing environment:
```bash
# Shut down the environment
docker compose down

# Remove local version of the image
docker rmi dev-environment:latest
```

2. Make your changes to the `Dockerfile`.

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
- This is an experimental distribution method.
- Availability depends on peer seeding.
- Consider building locally for production use.
