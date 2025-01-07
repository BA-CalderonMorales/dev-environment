#!/bin/bash
set -e

echo "🔄 Updating repository with latest release info..."

# Get the latest version and magnet link
VERSION=$(date +%Y%m%d%H%M)
MAGNET_LINK=$(cat magnet.txt)

# Update the magnet link in the repository
echo "$MAGNET_LINK" > magnet.txt

# Commit and push if there are changes
if git diff --quiet magnet.txt; then
    echo "No changes to magnet link"
else
    git config --global user.name "GitHub Actions"
    git config --global user.email "actions@github.com"
    
    git add magnet.txt
    git commit -m "chore: update magnet link for version ${VERSION}"
    git push
fi

echo "✅ Repository updated successfully" 