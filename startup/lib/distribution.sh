#!/bin/bash

# Function to handle DockerHub distribution
handle_dockerhub_distribution() {
    echo "Pulling image from DockerHub..."
    
    # Check for rate limits if not in test environment
    if [ -z "$SIMULATE_DOCKERHUB_RATE_LIMIT" ]; then
        # Try to pull the image
        if docker pull cmoe640/dev-environment:latest; then
            echo "Starting environment..."
            docker compose up -d
            return 0
        else
            echo "DockerHub pull failed. Rate limit may be exceeded."
            return 1
        fi
    else
        echo "DockerHub rate limit simulated."
        return 1
    fi
}

# Function to handle BitTorrent distribution
handle_bittorrent_distribution() {
    echo "Downloading via BitTorrent..."
    
    # Skip if force fail is set
    if [ "$FORCE_BITTORRENT_FAIL" = "true" ]; then
        echo "BitTorrent distribution forced to fail"
        return 1
    fi
    
    # Check for transmission-cli
    if ! command -v transmission-cli &> /dev/null; then
        echo "Installing transmission-cli..."
        sudo apt-get update && sudo apt-get install -y transmission-cli
    fi

    # Get latest magnet link and checksum
    MAGNET_LINK=$(curl -s https://raw.githubusercontent.com/BA-CalderonMorales/dev-environment/main/distributions/bittorrent/magnet.txt)
    EXPECTED_CHECKSUM=$(curl -s https://raw.githubusercontent.com/BA-CalderonMorales/dev-environment/main/distributions/bittorrent/checksum.txt)
    
    if [ -z "$MAGNET_LINK" ] || [ -z "$EXPECTED_CHECKSUM" ]; then
        echo "Failed to fetch magnet link or checksum"
        return 1
    fi
    
    echo "Starting download..."
    transmission-cli "$MAGNET_LINK" --download-dir .
    
    if [ -f "dev-environment.tar" ]; then
        # Verify checksum
        ACTUAL_CHECKSUM=$(sha256sum dev-environment.tar | cut -d' ' -f1)
        if [ "$ACTUAL_CHECKSUM" != "$EXPECTED_CHECKSUM" ]; then
            echo "Checksum verification failed"
            rm dev-environment.tar
            return 1
        fi
        
        echo "Loading Docker image..."
        docker load < dev-environment.tar
        rm dev-environment.tar
        echo "Starting environment..."
        docker compose up -d
        return 0
    else
        echo "Download failed."
        return 1
    fi
}
