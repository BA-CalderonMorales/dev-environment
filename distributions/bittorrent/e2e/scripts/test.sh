#!/bin/bash
set -e

echo "🧪 Running BitTorrent Distribution E2E Tests..."

# Setup
TEST_DIR=$(mktemp -d)
cd $TEST_DIR

# Create projects directory
mkdir -p projects

# Copy docker-compose.yml and modify for test environment
cp $GITHUB_WORKSPACE/distributions/bittorrent/docker-compose.yml .

# Replace relative path with absolute path
sed -i "s|../../projects|$TEST_DIR/projects|g" docker-compose.yml

# Remove any potential .gitconfig mount that might be in the file
sed -i '/\.gitconfig/d' docker-compose.yml

# For E2E testing, we'll use the DockerHub image and tag it appropriately
echo "📥 Pulling and retagging Docker image for testing..."
docker pull cmoe640/dev-environment:latest
docker tag cmoe640/dev-environment:latest dev-environment:latest

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

# Test that init script exists and is executable
echo "📝 Verifying init script..."
docker exec dev-environment bash -c '
    test -x /usr/src/startup/init-project.sh
'

# Cleanup
echo "🧹 Cleaning up..."
docker compose down
docker rmi dev-environment:latest

echo "✅ BitTorrent E2E tests completed successfully"
