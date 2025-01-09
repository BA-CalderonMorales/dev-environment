#!/bin/bash
set -e

echo "🧪 Running BitTorrent Distribution E2E Tests..."

# Setup
TEST_DIR=$(mktemp -d)
cd $TEST_DIR

# Create necessary directories
mkdir -p artifacts/bittorrent

# Copy artifacts from the workflow's artifact directory
if [ -d "$GITHUB_WORKSPACE/artifacts/bittorrent" ]; then
    cp -r $GITHUB_WORKSPACE/artifacts/bittorrent/* artifacts/bittorrent/
else
    echo "❌ BitTorrent artifacts directory not found"
    exit 1
fi

# Create projects directory
mkdir -p projects

# Copy required files
cp -r $GITHUB_WORKSPACE/startup .
cp -r $GITHUB_WORKSPACE/artifacts/bittorrent/* .
cp $GITHUB_WORKSPACE/distributions/bittorrent/docker-compose.yml .

# Replace relative path with absolute path
sed -i "s|../../projects|$TEST_DIR/projects|g" docker-compose.yml
sed -i '/\.gitconfig/d' docker-compose.yml

echo "🔍 Testing BitTorrent Distribution Path..."

# Test BitTorrent-first approach
export PREFER_BITTORRENT=true
chmod +x startup/start-dev.sh

echo "📥 Testing BitTorrent download..."
if ! ./startup/start-dev.sh; then
    echo "❌ BitTorrent distribution failed"
    exit 1
fi

# Verify BitTorrent download worked
if ! docker images | grep -q "dev-environment"; then
    echo "❌ BitTorrent image load failed"
    exit 1
fi

# Test Development Tools
echo "🛠️ Verifying development tools..."
docker exec dev-environment bash -c '
    echo "Node.js: $(node --version)" &&
    echo "Go: $(go version)" &&
    echo "Rust: $(cargo --version)" &&
    echo "Git: $(git --version)"
'

# Cleanup first test
docker compose down
docker rmi dev-environment:latest 2>/dev/null || true

echo "🔄 Testing DockerHub fallback..."
export FORCE_BITTORRENT_FAIL=true
if ! ./startup/start-dev.sh; then
    echo "❌ DockerHub fallback failed"
    exit 1
fi

# Cleanup
echo "🧹 Cleaning up..."
docker compose down
docker rmi dev-environment:latest 2>/dev/null || true
docker rmi cmoe640/dev-environment:latest 2>/dev/null || true

echo "✅ BitTorrent distribution tests completed successfully"
