#!/usr/bin/env bash

# Exit immediately if a command exits with a non-zero status
set -e

# Script to slim down Docker images using docker-slim
# Usage: ./slim-docker-image.sh <image-name>:<tag>

# Default values
IMAGE_NAME="cmoe640/dev-environment:latest"
OUTPUT_TAG="slim"
INCLUDE_PATHS="/usr/src/projects /usr/src/startup /home/devuser/.bashrc /home/devuser/.cargo"
PRESERVE_PATH_PERMS=true

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --image=*)
      IMAGE_NAME="${1#*=}"
      shift
      ;;
    --output-tag=*)
      OUTPUT_TAG="${1#*=}"
      shift
      ;;
    --help)
      echo "Usage: $0 [options]"
      echo "Options:"
      echo "  --image=IMAGE_NAME     Docker image to slim (default: cmoe640/dev-environment:latest)"
      echo "  --output-tag=TAG       Tag for the slimmed image (default: slim)"
      echo "  --help                 Show this help message"
      exit 0
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

OUTPUT_IMAGE="${IMAGE_NAME%:*}:${OUTPUT_TAG}"

# Check if docker-slim is installed
if ! command -v docker-slim &> /dev/null; then
    echo "❌ docker-slim not found. Installing..."
    
    # Install docker-slim
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        curl -sL https://raw.githubusercontent.com/docker-slim/docker-slim/master/scripts/install-dockerslim.sh | sudo -E bash -
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        brew install docker-slim
    else
        echo "❌ Unsupported OS. Please install docker-slim manually: https://github.com/docker-slim/docker-slim"
        exit 1
    fi
fi

echo "🔍 Analyzing image: $IMAGE_NAME"
echo "🏷️ Output image will be: $OUTPUT_IMAGE"

# Make sure the image exists locally
docker pull $IMAGE_NAME || { echo "❌ Failed to pull image: $IMAGE_NAME"; exit 1; }

# Run docker-slim
echo "🔄 Running docker-slim..."
docker-slim build \
    --http-probe=false \
    --continue-after=900 \
    --preserve-path-perms=$PRESERVE_PATH_PERMS \
    $(echo "$INCLUDE_PATHS" | xargs -n1 echo "--include-path") \
    --tag $OUTPUT_IMAGE \
    --dockerfile-tweaks-only=false \
    --expose 22 \
    --cmd "/bin/bash" \
    --entrypoint "/bin/bash" \
    $IMAGE_NAME

# Report image sizes
ORIGINAL_SIZE=$(docker image ls --format "{{.Size}}" $IMAGE_NAME)
SLIM_SIZE=$(docker image ls --format "{{.Size}}" $OUTPUT_IMAGE)

echo "📊 Results:"
echo "  Original Image: $ORIGINAL_SIZE"
echo "  Slimmed Image: $SLIM_SIZE"
echo "✅ Done! You can now use the slimmed image: $OUTPUT_IMAGE"
echo ""
echo "🚀 Run with: docker run -it $OUTPUT_IMAGE"
