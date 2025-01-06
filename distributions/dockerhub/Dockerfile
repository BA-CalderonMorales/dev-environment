# syntax=docker/dockerfile:1
FROM ubuntu:latest

ARG NODE_VERSION=22.3.0
ARG GO_VERSION=1.22.4

# Create user first
RUN useradd -m -s /bin/bash devuser

# Install Node.js for devuser
USER devuser
WORKDIR /home/devuser
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash && \
    . ~/.nvm/nvm.sh && \
    nvm install ${NODE_VERSION} && \
    nvm use ${NODE_VERSION} && \
    echo 'export NVM_DIR="$HOME/.nvm"' >> ~/.bashrc && \
    echo '[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"' >> ~/.bashrc && \
    echo '[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"' >> ~/.bashrc

# Switch back to root for system installations
USER root

# Install system dependencies
RUN apt-get update && \
    apt-get install -y wget curl git sqlite3

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

# Add welcome message
RUN echo 'echo "╔════════════════════════════════════════╗"' >> ~/.bashrc && \
    echo 'echo "║    Development Environment Versions    ║"' >> ~/.bashrc && \
    echo 'echo "╠════════════════════════════════════════╣"' >> ~/.bashrc && \
    echo 'echo "║ Node.js: $(node --version)"' >> ~/.bashrc && \
    echo 'echo "║ Go:      $(go version | cut -d" " -f3)"' >> ~/.bashrc && \
    echo 'echo "║ Rust:    $(rustc --version | cut -d" " -f2)"' >> ~/.bashrc && \
    echo 'echo "║ Git:     $(git --version | cut -d" " -f3)"' >> ~/.bashrc && \
    echo 'echo "║ SQLite:  $(sqlite3 --version | cut -d" " -f1)"' >> ~/.bashrc && \
    echo 'echo "╠════════════════════════════════════════╣"' >> ~/.bashrc && \
    echo 'echo "║ Working Directory: $(pwd)"' >> ~/.bashrc && \
    echo 'echo "╚════════════════════════════════════════╝\n"' >> ~/.bashrc

# Set final working directory
WORKDIR /usr/src/projects