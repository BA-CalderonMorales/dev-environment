#!/bin/bash
set -e

echo "🧪 Running DockerHub Distribution E2E Tests..."

# Setup
TEST_DIR=$(mktemp -d)
cd $TEST_DIR

# Create necessary directories
mkdir -p artifacts/dockerhub

# Copy artifacts from the workflow's artifact directory
if [ -d "$GITHUB_WORKSPACE/artifacts/dockerhub" ]; then
    cp -r $GITHUB_WORKSPACE/artifacts/dockerhub/* artifacts/dockerhub/
else
    echo "❌ DockerHub artifacts directory not found"
    exit 1
fi

# Verify image-info.json exists and contains required fields
if [ ! -f "artifacts/dockerhub/image-info.json" ]; then
    echo "❌ image-info.json not found"
    exit 1
fi

# Parse and verify image info
IMAGE_TAG=$(jq -r '.image' artifacts/dockerhub/image-info.json)
if [ -z "$IMAGE_TAG" ]; then
    echo "❌ Invalid image-info.json: missing image tag"
    exit 1
fi

# Create projects directory
mkdir -p projects

# Copy required files
cp -r $GITHUB_WORKSPACE/startup .
cp $GITHUB_WORKSPACE/distributions/dockerhub/docker-compose.yml .

# Replace relative path with absolute path
sed -i "s|../../projects|$TEST_DIR/projects|g" docker-compose.yml
sed -i '/\.gitconfig/d' docker-compose.yml

echo "🔍 Testing DockerHub Distribution..."

# Test container startup
echo "📦 Testing container startup..."
docker compose up -d
sleep 10

# Verify container is running
if ! docker ps | grep -q "dev-environment"; then
    echo "❌ Container failed to start"
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

# Enhance cleanup section
cleanup() {
    echo "🧹 Running cleanup..."
    docker compose down -v 2>/dev/null || true
    docker rmi dev-environment:latest 2>/dev/null || true
    docker rmi cmoe640/dev-environment:latest 2>/dev/null || true
    rm -rf $TEST_DIR 2>/dev/null || true
}

# Ensure cleanup runs even if script fails
trap cleanup EXIT

# Remove explicit cleanup at end since trap handles it

echo "✅ DockerHub distribution tests completed successfully" 