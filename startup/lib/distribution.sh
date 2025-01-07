#!/bin/bash

# Function to handle DockerHub distribution
handle_dockerhub_distribution() {
    echo "Pulling image from DockerHub..."
    # Reference to existing DockerHub README.md for version info:
    startLine: 82
    endLine: 88
}

# Function to handle BitTorrent distribution
handle_bittorrent_distribution() {
    echo "Downloading via BitTorrent..."
    
    # Check for transmission-cli
    if ! command -v transmission-cli &> /dev/null; then
        echo "Installing transmission-cli..."
        sudo apt-get update && sudo apt-get install -y transmission-cli
    fi

    # Get latest magnet link from repository
    MAGNET_LINK=$(curl -s https://raw.githubusercontent.com/BA-CalderonMorales/dev-environment/main/distributions/bittorrent/magnet.txt)
    
    echo "Starting download..."
    transmission-cli "$MAGNET_LINK" --download-dir .
    
    if [ -f "dev-environment.tar" ]; then
        echo "Loading Docker image..."
        docker load < dev-environment.tar
        rm dev-environment.tar
        echo "Starting environment..."
        docker compose up -d
    else
        echo "Download failed. Falling back to DockerHub..."
        handle_dockerhub_distribution
    fi
}
