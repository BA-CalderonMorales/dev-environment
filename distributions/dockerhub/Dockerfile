# syntax=docker/dockerfile:1
FROM ubuntu:latest

ARG NODE_VERSION=22.12.0
ARG GO_VERSION=1.22.4

# Install system dependencies first
RUN apt-get update && \
    apt-get install -y wget curl git sqlite3 ca-certificates

# Install Node.js directly
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g npm@10.8.1 && \
    npm cache clean --force

# Create user and setup directories
RUN useradd -m -s /bin/bash devuser && \
    mkdir -p /usr/src/projects && \
    mkdir -p /usr/src/startup && \
    chown -R devuser:devuser /usr/src/projects && \
    chown -R devuser:devuser /usr/src/startup

# Install Go
RUN curl -sL https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz | tar -xz -C /usr/local
ENV PATH="/usr/local/go/bin:${PATH}"

# Switch to devuser for Rust installation
USER devuser
WORKDIR /home/devuser

# Install Rust and source environment
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    echo 'source "$HOME/.cargo/env"' >> ~/.bashrc

# Add colorful welcome message with emojis
RUN echo 'echo ""' >> ~/.bashrc && \
    echo 'echo "🛠️  Development Environment Versions"' >> ~/.bashrc && \
    echo 'echo "' >> ~/.bashrc && \
    echo 'echo -e "\033[1;32m⚡ Node.js:\033[0m  $(node --version)"' >> ~/.bashrc && \
    echo 'echo -e "\033[1;36m🔷 Go:\033[0m       $(go version | cut -d" " -f3)"' >> ~/.bashrc && \
    echo 'echo -e "\033[1;31m🦀 Rust:\033[0m     $(rustc --version | cut -d" " -f2)"' >> ~/.bashrc && \
    echo 'echo -e "\033[1;35m📦 Git:\033[0m      $(git --version | cut -d" " -f3)"' >> ~/.bashrc && \
    echo 'echo -e "\033[1;33m💾 SQLite:\033[0m   $(sqlite3 --version | cut -d" " -f1)"' >> ~/.bashrc && \
    echo 'echo ""' >> ~/.bashrc && \
    echo 'echo -e "\033[1;34m📂 Working Directory:\033[0m $(pwd)"' >> ~/.bashrc && \
    echo 'echo ""' >> ~/.bashrc

# Switch back to root for file operations
USER root

# Copy and set permissions for startup scripts
COPY startup/ /usr/src/startup/
RUN chmod -R 755 /usr/src/startup && \
    chown -R devuser:devuser /usr/src/startup && \
    chmod -R +x /usr/src/startup/*.sh

# Switch back to devuser for final setup
USER devuser
WORKDIR /usr/src/projects