# Contributing to Sadgi Protocol

First off, thank you for considering contributing to the Sadgi Protocol! It's people like you that make Sadgi a great platform for privacy-preserving credit and identity solutions.

## Getting Started

1. **Fork the repository** on GitHub.
2. **Clone your fork** locally: `git clone https://github.com/your-username/sadgi-protocol.git`
3. **Set up the development environment**: We recommend using the provided `scripts/local-ci.sh` to ensure your local setup matches CI.
4. **Create a branch**: `git checkout -b feature/your-feature-name`

## Development Workflow

### Rust Workspaces
This repository contains two primary Rust workspaces:
- `contracts/`: Soroban smart contracts.
- `prover/`: SP1 zkVM proof programs and host nodes.

Make sure you run `cargo check --workspace` and `cargo test --workspace` in both directories.

### Next.js Dashboard
The frontend lives in `dashboard/`. Use `npm install` and `npm run dev` to start it locally.

### Formatting and Linting
Before submitting a pull request, ensure your code passes our strict formatting and linting rules:
```bash
# In both root and prover/ directories:
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

## Pull Request Process

1. Ensure any new features include appropriate tests.
2. Update the `README.md` or `docs/` if your changes impact user-facing APIs or deployment.
3. Fill out the Pull Request Template completely.
4. Your PR will require a review from a code owner before it can be merged.
5. All CI status checks (including `cargo-deny`, `clippy`, and formatting) must pass.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).
