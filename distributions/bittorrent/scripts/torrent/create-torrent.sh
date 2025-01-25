#!/bin/bash
set -e

echo "🌱 Creating torrent file..."

# Create torrent with multiple trackers for redundancy
transmission-create \
    -o dev-environment.torrent \
    -t udp://tracker.opentrackr.org:1337 \
    -t udp://tracker.openbittorrent.com:6969 \
    -t udp://tracker.internetwarriors.net:1337 \
    -c "Development Environment Docker Image" \
    dev-environment.tar

echo "✅ Torrent file created" 