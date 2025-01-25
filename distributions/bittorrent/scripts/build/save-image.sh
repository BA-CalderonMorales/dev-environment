#!/bin/bash
set -e

echo "💾 Saving Docker image..."
docker save dev-environment:latest > dev-environment.tar
echo "✅ Docker image saved as dev-environment.tar" 