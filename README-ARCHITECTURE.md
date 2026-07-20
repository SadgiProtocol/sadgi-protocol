# Sadgi Protocol Architecture

Sadgi is composed of several interdependent layers, designed to securely bridge off-chain Zero-Knowledge computation with on-chain Soroban smart contracts.

## 1. The Developer SDK (`sdk/`)
A `no_std` Rust library that allows developers to easily construct `JobRequest` structures and define the cryptographic constraints (e.g., SP1 Image IDs) required for their programs.

## 2. The Compute Marketplace (`contracts/marketplace/`)
The economic heart of the protocol. Developers escrow XLM here. The decentralized Scheduler dynamically manages Prover stakes, matching capable machines to workloads, and enforcing penalties for missed deadlines.

## 3. The Prover Infrastructure (`infrastructure/prover-node/`)
The off-chain daemon operated by infrastructure providers. It constantly indexes the Stellar ledger for new `JobRequest` events, executes the target RISC-V binary, generates a `ProofReceipt`, and submits the transaction back to the Marketplace.

For detailed specifications on the architecture, please visit the [Official Documentation Site](https://sadgiprotocol.github.io/sadgi-protocol/).
