# Sadgi Protocol Whitepaper
**The Verifiable Compute Standard for Stellar**

## Abstract
The Sadgi Protocol bridges the gap between computationally heavy, privacy-preserving workloads and the deterministic, public nature of the Stellar blockchain. By leveraging Zero-Knowledge Virtual Machines (zkVMs) and the Soroban smart contract platform, Sadgi allows developers to execute complex logic off-chain and settle cryptographically guaranteed proofs on-chain. Built natively for the Stellar ecosystem, Sadgi operates without a speculative governance token—relying entirely on XLM to power a decentralized Compute Marketplace. 

The ultimate vision is for Sadgi to become the default trust layer for Stellar, enabling everything from private compliance checks to verifiable AI inference.

---

# Part 1: Why Sadgi?

## 1. Vision & Mission
**Sadgi is the verifiable compute protocol for Stellar, enabling developers to execute private and computationally intensive workloads off-chain while settling cryptographic proofs on-chain.**

Our mission is to make zero-knowledge cryptography accessible to every developer building on Stellar, transforming it from a niche academic pursuit into standard, easy-to-use infrastructure.

## 2. A New Protocol Standard
Sadgi is not merely a smart contract or an application; it is a **New Protocol Standard**. 

Currently, Stellar developers lack a standardized way to verify off-chain computation. Sadgi introduces a vertically integrated stack to solve this:
- **Developer Platform**: Tooling and Starter Kits to build ZK apps in minutes.
- **Verification Infrastructure**: Standardized on-chain Verifier contracts.
- **Marketplace**: A decentralized clearinghouse for compute resources.
- **SDKs**: Rust abstractions over complex polynomial commitments.
- **Services**: Turnkey Prover Node daemons for infrastructure providers.
- **Applications**: Real-world use cases (Identity, Credit) built on the standard.

## 3. The Three Macro Catalysts
The convergence of three macro trends makes Sadgi inevitable today:
1. **The Rise of AI**: AI models require massive compute that cannot run on a blockchain. Trustless AI requires verifiable off-chain execution.
2. **Growing Privacy Demands**: Financial and healthcare applications cannot post sensitive data (like KYC or medical records) to public ledgers. 
3. **Soroban's Maturation**: The launch of Stellar's Rust-based smart contract platform provides the perfect deterministic settlement layer for zero-knowledge proofs.

---

# Part 2: How It Works

## 4. Protocol Architecture
The Sadgi Protocol operates as a multi-layered architecture bridging off-chain execution and on-chain settlement:

1. **Developer SDK**: A Rust `no_std` library to easily request and verify proofs.
2. **Compute Marketplace**: The decentralized clearinghouse where developers fund jobs and Provers compete to compute them.
3. **Proof Engine**: The off-chain zkVM (initially SP1) that generates the cryptographic receipts.
4. **Soroban Verifier**: The on-chain settlement layer that verifies the mathematical soundness of the receipts.

### The Lifecycle of a Proof
1. **Request**: A Developer submits a `JobRequest` to the Soroban Marketplace, escrowing an XLM bounty.
2. **Matching**: The Compute Marketplace assigns the request to one or more Prover nodes based on capacity and reputation.
3. **Execution**: The Prover executes the off-chain verifiable computation inside a zkVM and generates a `ProofReceipt`.
4. **Verification**: Soroban verifies the `ProofReceipt` on-chain.
5. **Settlement**: The Developer's XLM bounty is unlocked and transferred to the successful Prover.

## 5. Technology Choices
- **Why Rust?**: Rust offers memory safety and is the native language for both Soroban smart contracts and SP1 guest programs. Developers only need to know one language.
- **Why zkVMs?**: Unlike circuit-based ZK systems (zk-SNARKs/STARKs), zkVMs allow developers to write standard software without needing to understand custom cryptographic circuits or polynomial arithmetic.
- **Why Stellar?**: Stellar offers near-instant finality, incredibly low transaction fees, and a robust financial ecosystem (Anchors, USDC) perfectly suited for a high-throughput Compute Marketplace.

## 6. The Compute Marketplace
The Compute Marketplace is Sadgi's decentralized scheduling engine, optimizing for latency and throughput.
- **Matching & Redundancy**: Jobs can be assigned to multiple Provers simultaneously. The fastest valid proof submitted claims the bounty, ensuring ultra-low latency for critical operations.
- **Capacity Advertising**: Provers advertise their CPU/RAM capabilities on-chain, allowing the Scheduler to route computationally heavy jobs efficiently.

---

# Part 3: Trust & Security

## 7. Design Principles
Sadgi is governed by a strict set of trust-minimizing design tenets:
- **Stellar Native & XLM First**: Sadgi does not have a speculative protocol token. All economics and fees are denominated in XLM.
- **Privacy by Default**: Data used to generate proofs remains strictly off-chain. Only the cryptographic Receipt and public journal are submitted to Soroban.
- **Backend Agnostic**: While initially built for SP1, the protocol is designed to support any zkVM backend in the future via a standard interface.
- **Minimal Trust Governance**: Governance is designed to manage parameters, not to custody user funds or manipulate proofs.

## 8. The Sadgi Standards
To create a thriving ecosystem, Sadgi introduces a family of open standards:
- **Sadgi Receipt Standard**: The common serialization format for cryptographic proofs on Stellar.
- **Sadgi Program Standard**: The boilerplate standard for writing compatible zkVM guest programs.
- **Sadgi Job Standard**: The on-chain representation of compute requests, including Priority and Redundancy metrics.

## 9. Economics & Incentives
Sadgi relies entirely on **XLM** to align incentives and deter malicious actors.
- **Dynamic Staking**: Provers must lock an XLM stake that scales with their concurrent job load.
- **Progressive Discipline**: If a Prover misses a deadline, they suffer a Reputation penalty and temporary suspension, allowing them to recover from hardware crashes. 
- **Slashing**: Slashing (forfeiting the XLM stake) is strictly reserved for malicious cryptographic fraud (submitting fake proofs).

## 10. Security & Threat Model
- **Fake Receipts (Cryptographic Attacks)**: Mitigated by the Soroban verifier, which mathematically rejects any invalid zkVM receipt. Malicious Provers lose their entire XLM stake instantly.
- **Spam (Marketplace Attacks)**: Mitigated by minimum XLM escrows required to queue a Job.
- **Front-Running**: Mitigated by assigning specific jobs to specific Provers. Even in redundant "fastest-wins" modes, only authorized Provers can claim the bounty.

---

# Part 4: Adoption & Ecosystem

## 11. Real Use Cases
Sadgi unlocks use cases previously impossible on a public blockchain:
- **Age Verification**: Proving a user is 18+ without revealing their birthdate to the dApp.
- **Academic Credentials**: Trustlessly verifying a university degree for employment smart contracts.
- **Payroll Verification**: Proving income requirements for DeFi lending protocols without leaking salary data.
- **AI Inference Verification**: Proving that an AI model produced a specific output based on a specific input, entirely off-chain.

## 12. Ecosystem Strategy (The Flywheel)
The Sadgi Ecosystem is driven by a powerful flywheel effect:
As developers build more Reference Programs, more applications launch. This increases Proof Requests, driving demand for Provers. More Provers increases competition, lowering costs and latency, which in turn attracts even more developers to the Stellar ecosystem.

## 13. Governance Roadmap
Sadgi operates a tokenless, Progressive Decentralization model.
1. **Core Team**: A 3-of-5 Multisig to bootstrap the network.
2. **Foundation Stewardship**: Transitioning control to a non-profit entity for stability.
3. **Technical Council**: Expanding to verified node operators and ecosystem developers.
4. **Community Governance**: Fully decentralized, reputation-based stewardship using the Sadgi Improvement Proposal (SIP) lifecycle.

## 14. Vision 2030
By 2030, every Stellar wallet will verify zero-knowledge proofs natively. Every Soroban contract will have the ability to consume Sadgi receipts. Governments will issue verifiable credentials through Sadgi, banks will verify compliance privately, and AI systems will produce verifiable outputs. 

Sadgi will be the invisible, default trust layer powering the Stellar ecosystem.
