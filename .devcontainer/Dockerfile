# Use a Rust base image (version 1.84)
FROM rust:1.84.0-bookworm

# Install Node.js, pnpm, and other necessary tools
RUN curl -fsSL https://deb.nodesource.com/setup_21.x | bash - && \
  apt-get install -y nodejs && \
  npm install -g pnpm

# Set up development tools
RUN cargo install sqlx-cli

# Set the working directory
WORKDIR /workspace/pennysage

# Expose any required ports (e.g., frontend, backend)
EXPOSE 80 3000