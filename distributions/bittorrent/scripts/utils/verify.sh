#!/bin/bash
set -e

verify_files() {
    local required_files=(
        "dev-environment.tar"
        "dev-environment.torrent"
        "magnet.txt"
    )

    for file in "${required_files[@]}"; do
        if [[ ! -f "$file" ]]; then
            echo "❌ Required file not found: $file"
            return 1
        fi
    done

    echo "✅ All required files present"
    return 0
}

verify_torrent() {
    if ! transmission-show dev-environment.torrent > /dev/null 2>&1; then
        echo "❌ Invalid torrent file"
        return 1
    fi

    echo "✅ Torrent file verified"
    return 0
}

# Run verifications if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    verify_files && verify_torrent
fi 