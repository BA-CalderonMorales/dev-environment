# syntax=docker/dockerfile:1
FROM ubuntu:latest

ARG NODE_VERSION=22.3.0
ARG GO_VERSION=1.22.4

# Install Node.js
RUN apt-get update && \
    apt-get install -y wget curl && \
    wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash && \
    . /root/.nvm/nvm.sh && \
    nvm install ${NODE_VERSION} && \
    nvm use ${NODE_VERSION}

# Add Node to PATH
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin:${PATH}"

# Install Go
RUN curl -sL https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz | tar -xz -C /usr/local

# Add Go to PATH
ENV PATH="/usr/local/go/bin:${PATH}"

# Install git and SQLite
RUN apt-get install -y git sqlite3

# Create projects directory and user
RUN mkdir -p /usr/src/projects && \
    useradd -m -s /bin/bash devuser && \
    chown -R devuser:devuser /usr/src/projects

# Install Rust
USER root
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/devuser/.cargo/bin:${PATH}"

# Switch to devuser
USER devuser

# Set working directory
WORKDIR /usr/src/projects