#!/bin/bash
set -euo pipefail

# Ensure we're in the direct_download directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}/.."

# Create temp directory for package contents
TEMP_DIR=$(mktemp -d)
trap 'rm -rf ${TEMP_DIR}' EXIT

# Copy required files to temp directory
cp -r startup "${TEMP_DIR}/"

# Create tar archive
tar -czf dev-environment.tar -C "${TEMP_DIR}" .

# Clean up temp directory (trap will handle this)
