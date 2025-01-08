#!/bin/bash
set -e

# Color codes for better visibility
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Parse command line arguments
TEST_MODE=false
LOCAL_TEST=false
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --test-mode) TEST_MODE=true; shift ;;
        --local) LOCAL_TEST=true; shift ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
done

echo -e "${GREEN}Starting development environment setup...${NC}"

# Source the distribution handling functions
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/lib/distribution.sh"

# Check for required tools
check_requirements() {
    local missing_tools=()
    
    if ! command -v docker &> /dev/null; then
        missing_tools+=("docker")
    fi
    
    if ! command -v docker-compose &> /dev/null; then
        missing_tools+=("docker-compose")
    fi
    
    if [ ${#missing_tools[@]} -ne 0 ]; then
        echo -e "${RED}Error: Missing required tools: ${missing_tools[*]}${NC}"
        echo "Please install the missing tools and try again."
        exit 1
    fi
}

# Determine distribution method
determine_distribution() {
    if [ "$TEST_MODE" = "true" ] && [ "$LOCAL_TEST" = "true" ]; then
        echo -e "${YELLOW}Running in local test mode${NC}"
        # Add specific test behavior here
        return 0
    fi

    if [ "$PREFER_BITTORRENT" = "true" ]; then
        echo -e "${YELLOW}BitTorrent distribution preferred${NC}"
        if handle_bittorrent_distribution; then
            return 0
        else
            if [ "$TEST_MODE" = "true" ]; then
                echo -e "${RED}BitTorrent distribution failed during test${NC}"
                exit 1
            fi
            echo -e "${YELLOW}BitTorrent distribution failed, falling back to DockerHub...${NC}"
        fi
    fi

    # Fallback to DockerHub
    if ! handle_dockerhub_distribution; then
        if [ "$TEST_MODE" = "true" ]; then
            echo -e "${RED}DockerHub distribution failed during test${NC}"
            exit 1
        fi
        echo -e "${RED}All distribution methods failed${NC}"
        exit 1
    fi
}

# Verify environment
verify_environment() {
    local max_attempts=5
    local attempt=1
    local delay=5

    echo -e "${GREEN}Verifying environment...${NC}"
    
    while [ $attempt -le $max_attempts ]; do
        if docker ps | grep -q "dev-environment"; then
            echo -e "${GREEN}Environment verification successful!${NC}"
            return 0
        fi
        
        if [ $attempt -lt $max_attempts ]; then
            echo -e "${YELLOW}Attempt $attempt/$max_attempts: Environment not ready, waiting ${delay}s...${NC}"
            sleep $delay
        fi
        
        attempt=$((attempt + 1))
    done

    echo -e "${RED}Failed to verify environment after $max_attempts attempts${NC}"
    return 1
}

# Main execution
main() {
    check_requirements
    determine_distribution
    verify_environment
}

# Execute main function
main "$@"
