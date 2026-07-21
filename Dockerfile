# ==============================================================================
# Builder Stage
# ==============================================================================
FROM rust:slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libdbus-1-dev \
    libudev-dev \
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

# Standard OCI Labels
LABEL org.opencontainers.image.title="Sadgi Prover Node" \
      org.opencontainers.image.description="SP1-based Zero-Knowledge Prover Node for Sadgi Protocol" \
      org.opencontainers.image.vendor="Sadgi Protocol"

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd -r sadgi && useradd -r -g sadgi sadgi \
    && chown -R sadgi:sadgi /app

# Copy the compiled Prover daemon from the builder
COPY --from=builder /workspace/prover/target/release/sadgi-prover-node /app/sadgi-prover-node

USER sadgi

EXPOSE 8080

# Health check to ensure the prover API is responsive
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

# Default command runs the prover node
CMD ["/app/sadgi-prover-node", "start"]
