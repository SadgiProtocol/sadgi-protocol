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

# Install RISC Zero toolchain (for building programs)
RUN cargo binstall -y cargo-risczero
RUN cargo risczero install

WORKDIR /workspace

# Copy workspace manifests
COPY Cargo.toml Cargo.lock ./
COPY contracts/ contracts/
COPY programs/ programs/
COPY core/ core/
COPY sdk/ sdk/
COPY cli/ cli/

# Build Soroban Contracts
RUN cargo build --manifest-path contracts/marketplace/Cargo.toml --target wasm32-unknown-unknown --release

# Build Canonical Programs
# (Note: RISC Zero programs require specific build steps, normally handled by cargo risczero build)
RUN cargo build --manifest-path programs/hello_world/Cargo.toml --release
RUN cargo build --manifest-path programs/hash_verification/Cargo.toml --release
RUN cargo build --manifest-path programs/identity/Cargo.toml --release
RUN cargo build --manifest-path programs/credit/Cargo.toml --release

# Build Prover Daemon (and CLI)
RUN cargo build --release

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
COPY --from=builder /workspace/target/release/sadgi-prover-node /app/sadgi-prover-node

EXPOSE 8080

# Default command runs the prover node
CMD ["/app/sadgi-prover-node", "start"]
