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

# Add Node to PATH and setup NVM for devuser
ENV NVM_DIR=/root/.nvm
ENV NODE_PATH=$NVM_DIR/versions/node/v${NODE_VERSION}/lib/node_modules
ENV PATH=$NVM_DIR/versions/node/v${NODE_VERSION}/bin:$PATH

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

# Install Rust for devuser
USER devuser
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/devuser/.cargo/bin:${PATH}"

# Copy NVM setup to devuser
RUN cp -r /root/.nvm /home/devuser/ && \
    echo 'export NVM_DIR="$HOME/.nvm"' >> /home/devuser/.bashrc && \
    echo '[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"' >> /home/devuser/.bashrc && \
    echo '[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"' >> /home/devuser/.bashrc

# Add welcome message with all tool versions
RUN echo 'echo "╔════════════════════════════════════════╗"' >> /home/devuser/.bashrc && \
    echo 'echo "║    Development Environment Versions    ║"' >> /home/devuser/.bashrc && \
    echo 'echo "╠════════════════════════════════════════╣"' >> /home/devuser/.bashrc && \
    echo 'echo "║ Node.js: $(node --version)"' >> /home/devuser/.bashrc && \
    echo 'echo "║ Go:      $(go version | cut -d" " -f3)"' >> /home/devuser/.bashrc && \
    echo 'echo "║ Rust:    $(rustc --version | cut -d" " -f2)"' >> /home/devuser/.bashrc && \
    echo 'echo "║ Git:     $(git --version | cut -d" " -f3)"' >> /home/devuser/.bashrc && \
    echo 'echo "║ SQLite:  $(sqlite3 --version | cut -d" " -f1)"' >> /home/devuser/.bashrc && \
    echo 'echo "╠════════════════════════════════════════╣"' >> /home/devuser/.bashrc && \
    echo 'echo "║ Working Directory: $(pwd)"' >> /home/devuser/.bashrc && \
    echo 'echo "╚════════════════════════════════════════╝\n"' >> /home/devuser/.bashrc

# Set working directory
WORKDIR /usr/src/projects