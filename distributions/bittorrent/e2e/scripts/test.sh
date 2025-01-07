#!/bin/bash
set -e

echo "🧪 Running BitTorrent Distribution E2E Tests..."

# Setup
TEST_DIR=$(mktemp -d)
cd $TEST_DIR

# Copy necessary files
cp $GITHUB_WORKSPACE/distributions/bittorrent/dev-environment.tar .
cp $GITHUB_WORKSPACE/distributions/bittorrent/docker-compose.yml .

# Load image
echo "📥 Loading Docker image from torrent..."
docker load < dev-environment.tar

# Test Container Startup
echo "📦 Testing container startup..."
docker compose up -d
sleep 10

# Test Development Tools
echo "🛠️ Verifying development tools..."
docker exec dev-environment bash -c '
    echo "Node.js: $(node --version)" &&
    echo "Go: $(go version)" &&
    echo "Rust: $(cargo --version)" &&
    echo "Git: $(git --version)"
'

# Copy startup scripts into container
docker cp $GITHUB_WORKSPACE/startup/. dev-environment:/usr/src/startup/

# Test Project Initialization
echo "📁 Testing project initialization..."
docker exec dev-environment bash -c '
    cd /usr/src/projects &&
    /usr/src/startup/init-project.sh test-project-bt full-stack &&
    test -d test-project-bt
'

# Cleanup
echo "🧹 Cleaning up..."
docker compose down
docker rmi dev-environment:latest

echo "✅ BitTorrent E2E tests completed successfully"
