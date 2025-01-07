#!/bin/bash
set -e

echo "🎉 Creating GitHub release..."

VERSION=$(date +%Y%m%d%H%M)
MAGNET_LINK=$(cat magnet.txt)

gh release create "v${VERSION}" \
    --title "Dev Environment v${VERSION}" \
    --notes "Magnet Link: ${MAGNET_LINK}" \
    dev-environment.torrent

echo "✅ Release v${VERSION} created" 