# syntax=docker/dockerfile:1
FROM ubuntu:latest

ARG NODE_VERSION=22.3.0
ARG GO_VERSION=1.21.7

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

# Install other dependencies
RUN apt-get install -y git sqlite3

# Create a non-root user
RUN useradd -m -s /bin/bash devuser
USER devuser

# Set working directory
WORKDIR /home/devuser

# Install Rust
USER root
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/devuser/.cargo/bin:${PATH}"
USER devuser

# Copy project files - Adjust if necessary for initial setup
COPY --chown=devuser:devuser projects /usr/src/projects
