#!/bin/bash

get_version() {
    # Try to get version from Git first
    if command -v git >/dev/null 2>&1 && [ -d ".git" ]; then
        # Get latest tag, fallback to describe, finally to commit hash
        git describe --tags --abbrev=0 2>/dev/null || \
        git describe --always --dirty 2>/dev/null || \
        git rev-parse --short HEAD
    else
        # Read from VERSION file if Git not available
        cat VERSION 2>/dev/null || echo "unknown"
    fi
}

echo "Initializing development environment..."
echo "Version: $(get_version)"

# Check for custom configuration
if [ -f "/workspace/.devenv.conf" ]; then
    source /workspace/.devenv.conf
fi

# Initialize basic environment
mkdir -p ~/workspace
cd ~/workspace

echo "Environment ready! Use 'setup.sh --help' for more options"
