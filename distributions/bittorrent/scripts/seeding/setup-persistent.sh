#!/bin/bash
set -e

echo "🌱 Setting up persistent seeding..."

# Create systemd service for persistent seeding
sudo tee /etc/systemd/system/dev-env-seed.service << EOF
[Unit]
Description=Dev Environment Torrent Seeder
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$(pwd)
ExecStart=$(which transmission-cli) --seedratio 0 dev-environment.torrent
Restart=always

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable dev-env-seed
sudo systemctl start dev-env-seed

echo "✅ Persistent seeding service installed and started" 