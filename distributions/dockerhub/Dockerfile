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

# Create user
RUN useradd -m -s /bin/bash devuser

# Install Go
RUN curl -sL https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz | tar -xz -C /usr/local
ENV PATH="/usr/local/go/bin:${PATH}"

# Create and set projects directory
RUN mkdir -p /usr/src/projects && \
    chown -R devuser:devuser /usr/src/projects

# Switch to devuser for Rust installation
USER devuser
WORKDIR /home/devuser

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . $HOME/.cargo/env

# Add welcome message with centered text
RUN echo 'echo "╔══════════════════════════════════════════════════╗"' >> ~/.bashrc && \
    echo 'echo "║          Development Environment Versions         ║"' >> ~/.bashrc && \
    echo 'echo "╠══════════════════════════════════════════════════╣"' >> ~/.bashrc && \
    echo 'echo "║ Node.js:  v22.12.0                              ║"' >> ~/.bashrc && \
    echo 'echo "║ Go:       go1.22.4                              ║"' >> ~/.bashrc && \
    echo 'echo "║ Rust:     1.83.0                                ║"' >> ~/.bashrc && \
    echo 'echo "║ Git:      2.43.0                                ║"' >> ~/.bashrc && \
    echo 'echo "║ SQLite:   3.45.1                                ║"' >> ~/.bashrc && \
    echo 'echo "╠══════════════════════════════════════════════════╣"' >> ~/.bashrc && \
    echo 'echo "║ Working Directory: /usr/src/projects            ║"' >> ~/.bashrc && \
    echo 'echo "╚══════════════════════════════════════════════════╝\n"' >> ~/.bashrc

# Set final working directory
WORKDIR /usr/src/projects