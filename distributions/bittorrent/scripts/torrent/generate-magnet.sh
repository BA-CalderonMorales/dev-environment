#!/bin/bash
set -e

echo "🧲 Generating magnet link..."

MAGNET_LINK=$(transmission-show -m dev-environment.torrent)
echo "$MAGNET_LINK" > magnet.txt

echo "✅ Magnet link saved to magnet.txt" 