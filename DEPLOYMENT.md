# Sadgi Protocol Deployment Guide

This guide details the complete deployment strategy for deploying Sadgi Protocol to the Stellar Soroban network (Testnet & Mainnet) and operating the SP1 Prover Nodes.

## 1. Prerequisites
- `stellar-cli` (v21.0.1 or later)
- SP1 SDK (`sp1up`)
- Rust toolchain (stable)
- Next.js (Node 20+)

## 2. Soroban Smart Contract Deployment
To deploy the protocol securely, the contracts must be deployed in the correct sequence.

### A. Deploy Registry
```bash
stellar contract deploy --wasm target/wasm32v1-none/release/sadgi_registry.wasm --network testnet --source alice --alias sadgi-registry
```

### Deploy Verifier Contract
```bash
stellar contract deploy --wasm target/wasm32v1-none/release/sadgi_verifier.wasm --network testnet --source alice --alias sadgi-verifier
```

### Deploy Marketplace Contract
```bash
stellar contract deploy --wasm target/wasm32v1-none/release/sadgi_marketplace.wasm --network testnet --source alice --alias sadgi-marketplace
```

### D. Initialize Marketplace
Link the marketplace to the Native XLM token and the DAO Treasury:
```bash
stellar contract invoke --id sadgi-marketplace --network testnet --source alice -- initialize \
    --admin alice \
    --token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
    --treasury alice
```

## 3. SP1 ZK Prover Node Setup
The Prover node watches the blockchain for tasks, executes ELFs in the ZKVM, and submits Ed25519 signatures as Oracle bridges.

### Start the Daemon
```bash
cd prover/node
cargo run --release
```

## 4. Frontend Dashboard
Update `dashboard/app/page.tsx` with the newly deployed `MARKETPLACE_ID`.
```bash
cd dashboard
npm install
npm run build
npm start
```
