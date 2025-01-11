#!/bin/bash
set -e

echo "🔄 Testing Distribution Switching Mechanism..."

# Setup
TEST_DIR=$(mktemp -d)
cd $TEST_DIR

# Download start-dev.sh if not testing locally
if [ -z "$LOCAL_TEST" ]; then
    curl -O https://raw.githubusercontent.com/$GITHUB_REPOSITORY/main/startup/start-dev.sh
    chmod +x start-dev.sh
fi

# Test Cases
run_test_case() {
    local test_name=$1
    local env_vars=$2
    echo "📋 Running test case: $test_name"
    
    # Clean any existing containers/images
    docker compose down 2>/dev/null || true
    docker rmi dev-environment:latest 2>/dev/null || true
    docker rmi cmoe640/dev-environment:latest 2>/dev/null || true
    
    # Set environment variables for the test
    eval $env_vars
    
    # Run start-dev.sh
    ./start-dev.sh || {
        echo "❌ Test failed: $test_name"
        return 1
    }
    
    echo "✅ Test passed: $test_name"
    return 0
}

# Test Cases
echo "🧪 Running distribution switching tests..."

# Test 1: BitTorrent Failure -> DockerHub Fallback
run_test_case "BitTorrent Fallback" "export FORCE_BITTORRENT_FAIL=true"

# Test 2: DockerHub Rate Limit -> BitTorrent Fallback
run_test_case "DockerHub Fallback" "export SIMULATE_DOCKERHUB_RATE_LIMIT=true"

# Test 3: Both Methods Available -> Prefer BitTorrent
run_test_case "Preferred Distribution" "export PREFER_BITTORRENT=true"

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

echo "✅ Distribution switching tests completed" 