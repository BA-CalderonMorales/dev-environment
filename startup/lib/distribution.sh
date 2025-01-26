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

# Function to handle Direct Download distribution
handle_direct_download_distribution() {
    echo "Downloading via Direct Download..."
    
    # Skip if force fail is set
    if [ "$FORCE_DIRECT_DOWNLOAD_FAIL" = "true" ]; then
        echo "Direct Download distribution forced to fail"
        return 1
    fi
    
    # Get latest download URL and checksum
    DOWNLOAD_URL=$(curl -s https://raw.githubusercontent.com/BA-CalderonMorales/dev-environment/main/distributions/direct/url.txt)
    EXPECTED_CHECKSUM=$(curl -s https://raw.githubusercontent.com/BA-CalderonMorales/dev-environment/main/distributions/direct/checksum.txt)
    
    if [ -z "$DOWNLOAD_URL" ] || [ -z "$EXPECTED_CHECKSUM" ]; then
        echo "Failed to fetch download URL or checksum"
        return 1
    fi
    
    echo "Starting download..."
    curl -L -o dev-environment.tar "$DOWNLOAD_URL"
    
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
