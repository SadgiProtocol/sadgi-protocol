# Local Environment Setup

This guide walks you through running the entire Sadgi stack locally: contracts on Stellar Testnet, a prover node, and the SDK wired together.

## System Requirements

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | stable (≥ 1.78) | Soroban contracts + SP1 guest programs |
| Node.js | ≥ 20 | SDK and tooling |
| stellar-cli | ≥ 21.0 | Contract deployment & invocation |
| Docker | ≥ 24 | SP1 proof generation (GPU optional) |
| Git | any | Clone the repo |

## 1. Clone the Repository

```bash
git clone https://github.com/sadgi-protocol/sadgi-core.git
cd sadgi-core
```

## 2. Install Rust Components

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli
# Install SP1 toolchain
curl -L https://sp1.succinct.xyz | bash
sp1up
```

## 3. Configure Stellar CLI

```bash
stellar keys generate --global alice --network testnet
stellar keys address alice   # copy this address
# Fund with Friendbot
curl "https://friendbot.stellar.org?addr=$(stellar keys address alice)"
```

Set up your `.env` file in the project root:

```bash
cp .env.example .env
# Edit .env:
# STELLAR_NETWORK=testnet
# STELLAR_SECRET_KEY=<your secret key>
# PROVER_LOG_LEVEL=info
```

## 4. Build the Contracts

```bash
cd contracts
make build
# Produces:
#   target/wasm32-unknown-unknown/release/registry.wasm
#   target/wasm32-unknown-unknown/release/verifier.wasm
#   target/wasm32-unknown-unknown/release/marketplace.wasm
```

## 5. Deploy Contracts to Testnet

```bash
make deploy NETWORK=testnet DEPLOYER=alice
```

This script:
1. Uploads and instantiates `Registry`, `Verifier`, and `Marketplace`.
2. Calls `Marketplace.initialize(registry_addr, verifier_addr)`.
3. Writes contract addresses to `.contracts.json`.

## 6. Register a ZK Program

```bash
cargo run -p sadgi-cli -- register-program \
  --vk-path prover/programs/hash/vk.bin \
  --name "hash_proof_v1" \
  --network testnet
```

## 7. Start the Prover Node

```bash
docker compose up prover
# or, natively:
cargo run -p sadgi-prover -- \
  --network testnet \
  --keypair $STELLAR_SECRET_KEY \
  --programs hash,signature,threshold
```

The prover node will:
- Subscribe to `JobPosted` events from the Marketplace contract.
- Claim matching jobs, generate SP1 proofs, and submit the Groth16 SNARK.

## 8. Run the Test Suite

```bash
# Rust unit tests
cargo test --workspace

# Integration tests (requires running testnet + prover)
npm run test:integration
```

## Troubleshooting

**`wasm-opt` not found** — Install `binaryen`: `brew install binaryen` or `apt install binaryen`.

**Proof generation OOM** — SP1 requires ~8 GB RAM for medium circuits. Set `SP1_PROVER=mock` in `.env` to skip real proofs during development.

**Insufficient XLM** — Re-fund your account via Friendbot; contract storage rent requires ~1–2 XLM.
