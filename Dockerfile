# syntax=docker/dockerfile:1
ARG NODE_VERSION=22.3.0
ARG GO_VERSION=1.21

FROM ubuntu:latest

# Install Node.js
RUN apt-get update && \
    apt-get install -y wget curl && \
    wget https://nodejs.org/dist/v$NODE_VERSION/node-v$NODE_VERSION-linux-x64.tar.xz && \
    tar -xf node-v$NODE_VERSION-linux-x64.tar.xz -C /usr/local --strip-components=1 && \
    rm node-v$NODE_VERSION-linux-x64.tar.xz

# Install Go
RUN curl -sL https://go.dev/dl/go$GO_VERSION.linux-amd64.tar.gz | tar -xz -C /usr/local

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
