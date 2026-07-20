# ==============================================================================
# Builder Stage
# ==============================================================================
FROM rust:1.81-slim-bullseye AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    curl \
    git \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Install Soroban dependencies
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked soroban-cli

# Install SP1 toolchain
RUN curl -L https://sp1.succinct.xyz | bash
ENV PATH="/root/.sp1/bin:${PATH}"
RUN sp1up

WORKDIR /workspace

# Copy workspace manifests
COPY Cargo.toml Cargo.lock ./
COPY contracts/ contracts/
COPY core/ core/
COPY sdk/ sdk/
COPY cli/ cli/
COPY prover/ prover/

# Build Soroban Contracts
RUN cargo build --manifest-path contracts/marketplace/Cargo.toml --target wasm32-unknown-unknown --release

# Build SP1 Programs and Prover Daemon
# The prover is now in its own isolated workspace
RUN cd prover && cargo build --release

# ==============================================================================
# Prover Node Runtime Stage
# ==============================================================================
FROM debian:bullseye-slim AS prover

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled Prover daemon from the builder
# Since the prover is its own workspace, target is in /workspace/prover/target
COPY --from=builder /workspace/prover/target/release/sadgi-prover-node /app/sadgi-prover-node

EXPOSE 8080

# Default command runs the prover node
CMD ["/app/sadgi-prover-node", "start"]
