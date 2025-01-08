#!/bin/bash
set -e

# Color codes for better visibility
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

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
    if [ "$PREFER_BITTORRENT" = "true" ]; then
        echo -e "${YELLOW}BitTorrent distribution preferred${NC}"
        if handle_bittorrent_distribution; then
            return 0
        else
            echo -e "${YELLOW}BitTorrent distribution failed, falling back to DockerHub...${NC}"
        fi
    fi

    # Fallback to DockerHub
    handle_dockerhub_distribution
}

# Main execution
main() {
    check_requirements

    echo -e "${GREEN}Setting up development environment...${NC}"
    determine_distribution

    # Verify the environment is running
    if ! docker ps | grep -q "dev-environment"; then
        echo -e "${RED}Failed to start development environment${NC}"
        exit 1
    fi

    echo -e "${GREEN}Development environment is ready!${NC}"
    echo -e "To access the environment:"
    echo -e "  ${YELLOW}docker exec -it dev-environment bash${NC}"
    echo -e "To stop the environment:"
    echo -e "  ${YELLOW}docker compose down${NC}"
}

# Execute main function
main "$@"
