#!/bin/bash
set -e

echo "🧪 Running DockerHub Distribution E2E Tests..."

# Setup
TEST_DIR=$(mktemp -d)
cd $TEST_DIR

# Download docker-compose.yml
curl -O https://raw.githubusercontent.com/$GITHUB_REPOSITORY/main/distributions/dockerhub/docker-compose.yml

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
    /usr/src/startup/init-project.sh test-project full-stack &&
    test -d test-project
'

# Cleanup
echo "🧹 Cleaning up..."
docker compose down
docker rmi cmoe640/dev-environment:latest

echo "✅ DockerHub E2E tests completed successfully" 