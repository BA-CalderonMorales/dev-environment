#!/bin/bash
set -e

echo "🎉 Creating GitHub release..."

VERSION=$(date +%Y%m%d%H%M)
MAGNET_LINK=$(cat magnet.txt)

# Use gh CLI instead of direct API calls
gh release create "v${VERSION}" \
    --title "Dev Environment v${VERSION}" \
    --notes "Magnet Link: ${MAGNET_LINK}" \
    dev-environment.torrent \
    --repo "${GITHUB_REPOSITORY}"

echo "✅ Release v${VERSION} created" 