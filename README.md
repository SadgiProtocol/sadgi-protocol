# Sadgi Protocol
**The Verifiable Compute Protocol for Stellar**

Sadgi enables developers to execute private, computationally intensive workloads off-chain while settling mathematically guaranteed proofs directly on the Stellar blockchain.

It bridges the gap between privacy (e.g. Identity, Medical Records) or massive compute (e.g. AI inference) and the deterministic limits of smart contracts. By leveraging Zero-Knowledge Virtual Machines (zkVMs) and Soroban, Sadgi transforms Stellar into a fully verifiable ecosystem.

---

## Why Sadgi?
1. **Privacy by Default**: Process sensitive data off-chain without leaking it to the public ledger.
2. **Infinite Compute**: Run heavy AI inference or Big Data operations that would instantly exceed Soroban gas limits.
3. **XLM-Native**: There is no speculative governance token. The protocol runs purely on XLM and decentralized economics.

---

## 🗺️ Repository Architecture

This repository is a monorepo containing the entire Sadgi ecosystem:

```text
sadgi-protocol/
├── docs/             # Documentation, Architecture, and Whitepaper
├── contracts/        # Soroban Smart Contracts (Marketplace, Escrow)
├── core/             # Sadgi Standard Types (Receipts, Claims)
├── sdk/              # Rust Developer SDK
├── programs/         # Reference zkVM Programs (Identity, Credit)
├── infrastructure/   # Prover Node Executor
├── cli/              # Developer Command Line Interface
└── dashboard/        # Next.js Explorer and Marketplace UI
```

---

## ⚡ Quick Start (10 Minutes)

Run a complete end-to-end verifiable compute marketplace on your local machine.

### 1. Prerequisites
- [Rust](https://rustup.rs/) (>= 1.70)
- [Docker](https://www.docker.com/)
- Stellar CLI

### 2. Setup
Clone the repository and spin up the environment:
```bash
git clone https://github.com/SadgiProtocol/sadgi-protocol.git
cd sadgi-protocol

# Spins up the Soroban Localnet, a Mock Prover, and the Next.js Explorer
make dev
```

### 3. Build a "Hello World" Proof
Use the CLI to request your first zero-knowledge computation:
```bash
sadgi-cli request --program hello_world --bounty 10
```

You can view your job flowing through the Compute Marketplace by visiting `http://localhost:3000`.

---

## 📚 Documentation
For deep dives, tutorials, and protocol standards, visit our [Official Documentation](https://sadgiprotocol.github.io/sadgi-protocol/).
- [The Whitepaper](https://sadgiprotocol.github.io/sadgi-protocol/whitepaper.html)
- [Architecture & Threat Models](https://sadgiprotocol.github.io/sadgi-protocol/architecture.html)
- [SDK Reference](https://sadgiprotocol.github.io/sadgi-protocol/sdk.html)

---

## 🛣️ Roadmap & Community
Sadgi is an open standard designed to scale through community stewardship.
- **[Discussions](https://github.com/SadgiProtocol/sadgi-protocol/discussions)**: Join our community Office Hours or ask development questions.
- **SIPs**: Propose protocol upgrades via the Sadgi Improvement Proposal framework in our Issue Tracker.
- **Roadmap**: See our [Vision 2030](https://sadgiprotocol.github.io/sadgi-protocol/whitepaper.html#vision-2030) trajectory.

## License
Apache 2.0
