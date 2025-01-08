#!/bin/bash
set -e

echo "🧪 Running BitTorrent Distribution E2E Tests..."

# Setup
TEST_DIR=$(mktemp -d)
cd $TEST_DIR

# Create projects directory
mkdir -p projects

# Copy required files
cp $GITHUB_WORKSPACE/distributions/bittorrent/docker-compose.yml .
cp $GITHUB_WORKSPACE/startup/start-dev.sh .
cp -r $GITHUB_WORKSPACE/startup/lib .

# Replace relative path with absolute path
sed -i "s|../../projects|$TEST_DIR/projects|g" docker-compose.yml

# Remove any potential .gitconfig mount that might be in the file
sed -i '/\.gitconfig/d' docker-compose.yml

echo "🔍 Testing BitTorrent Distribution Path..."

# Test BitTorrent-first approach
export PREFER_BITTORRENT=true
chmod +x start-dev.sh

echo "📥 Attempting BitTorrent download..."
./start-dev.sh

# Verify the environment is running
if ! docker ps | grep -q "dev-environment"; then
    echo "❌ BitTorrent distribution test failed: Container not running"
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

# Test that init script exists and is executable
echo "📝 Verifying init script..."
docker exec dev-environment bash -c '
    test -x /usr/src/startup/init-project.sh
'

# Test fallback to DockerHub
echo "🔄 Testing DockerHub fallback..."
export FORCE_BITTORRENT_FAIL=true
docker compose down
docker rmi dev-environment:latest 2>/dev/null || true

# Should fall back to DockerHub
./start-dev.sh

# Verify fallback worked
if ! docker ps | grep -q "dev-environment"; then
    echo "❌ DockerHub fallback test failed: Container not running"
    exit 1
fi

# Cleanup
echo "🧹 Cleaning up..."
docker compose down
docker rmi dev-environment:latest 2>/dev/null || true
docker rmi cmoe640/dev-environment:latest 2>/dev/null || true

echo "✅ BitTorrent distribution tests completed successfully"
