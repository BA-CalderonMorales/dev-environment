#!/usr/bin/env bash

# Error handling
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Helper functions
error() {
    echo -e "${RED}ERROR: $1${NC}" >&2
    exit 1
}

info() {
    echo -e "${GREEN}INFO: $1${NC}"
}

warn() {
    echo -e "${YELLOW}WARNING: $1${NC}"
}

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    error "GitHub CLI (gh) is not installed. Please install it first:
    Windows: winget install GitHub.cli
    or visit: https://cli.github.com/"
fi

# Check if authenticated with GitHub
if ! gh auth status &> /dev/null; then
    error "Not authenticated with GitHub. Please run 'gh auth login' first."
fi

# Verify we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    error "Not in a git repository. Please navigate to a git repository first."
fi

# Get repository info
repo_name=$(gh repo view --json nameWithOwner -q .nameWithOwner 2>/dev/null || error "Failed to get repository info")
info "Repository: $repo_name"

# Confirm with user before proceeding
echo -e "\nThis will delete ALL workflow runs for $repo_name"
read -p "Are you sure? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    info "Operation cancelled"
    exit 0
fi

# Get and delete workflow runs
info "Fetching workflow runs..."
total_deleted=0
failed_deletes=0

gh run list --limit 100 --json databaseId -q '.[].databaseId' | while read -r run_id; do
    if [[ -n "$run_id" ]]; then
        echo -n "Deleting run ID: $run_id ... "
        if gh run delete "$run_id" 2>&1 | grep -q "Request to delete workflow run submitted"; then
            echo "✓"
            ((total_deleted++))
        else
            echo "✗"
            ((failed_deletes++))
        fi
    fi
done

# Summary
echo
info "Cleanup complete!"
echo "Successfully deleted: $total_deleted runs"
[[ $failed_deletes -gt 0 ]] && warn "Failed to delete: $failed_deletes runs"

# Keep terminal open (works in both Windows and Unix)
echo
echo "Press any key to continue..."
read -n 1 -s
