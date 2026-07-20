# Sadgi Protocol
**The Verifiable Compute Protocol for Stellar**

## 1. Executive Summary
The Sadgi Protocol bridges the gap between computationally heavy, privacy-preserving workloads and the deterministic, public nature of the Stellar blockchain. By leveraging Zero-Knowledge Virtual Machines (zkVMs) and the Soroban smart contract platform, Sadgi allows developers to execute complex logic off-chain and settle cryptographically guaranteed proofs on-chain. Built natively for the Stellar ecosystem, Sadgi operates without a speculative governance token—relying entirely on XLM to power a decentralized Compute Marketplace. The ultimate vision is for Sadgi to become the default trust layer for Stellar, enabling everything from private compliance checks to verifiable AI inference.

## 2. Vision & Mission
Sadgi is the verifiable compute protocol for Stellar, enabling developers to execute private and computationally intensive workloads off-chain while settling cryptographic proofs on-chain. 

Our mission is to make zero-knowledge cryptography accessible to every developer building on Stellar, transforming it from a niche academic pursuit into standard, easy-to-use infrastructure.

## 3. Why Now?
The convergence of three macro trends makes Sadgi inevitable today:
1. **The Rise of AI**: AI models require massive compute that cannot run on a blockchain. Trustless AI requires verifiable off-chain execution.
2. **Growing Privacy Demands**: Financial and healthcare applications cannot post sensitive data (like KYC or medical records) to public ledgers. 
3. **Soroban's Maturation**: The launch of Stellar's Rust-based smart contract platform provides the perfect deterministic settlement layer for zero-knowledge proofs.

## 4. Design Principles
Sadgi is governed by a strict set of design tenets:
- **Stellar Native & XLM First**: Sadgi does not have a speculative protocol token. All economics and fees are denominated in XLM.
- **Privacy by Default**: Data used to generate proofs remains strictly off-chain.
- **Backend Agnostic**: While initially built for RISC Zero, the protocol is designed to support any zkVM backend in the future.
- **Minimal Trust**: Governance is designed to manage parameters, not to custody user funds or manipulate proofs.

## 5. Protocol Architecture
The Sadgi Protocol operates as a multi-layered architecture:
1. **Applications**: Wallets, DAOs, and Enterprise software integrating Sadgi.
2. **Developer SDK**: A Rust `no_std` library to easily request and verify proofs.
3. **Compute Marketplace**: The decentralized clearinghouse where developers fund jobs and Provers compete to compute them.
4. **Proof Engine**: The off-chain zkVM (e.g., RISC Zero) that generates the cryptographic receipts.
5. **Soroban**: The on-chain settlement layer that verifies the receipts.

### The Protocol Lifecycle
1. **Developer** submits a `JobRequest` to the Soroban Marketplace, escrowing an XLM bounty.
2. **Compute Marketplace** matches the request to one or more Prover nodes based on capacity and reputation.
3. **Prover** executes the off-chain verifiable computation and generates a Receipt.
4. **Soroban** verifies the Receipt on-chain.
5. **Settlement**: The Developer's XLM bounty is unlocked and transferred to the Prover.

## 6. Technology Choices
- **Why Rust?**: Rust offers memory safety and is the native language for both Soroban smart contracts and RISC Zero guest programs.
- **Why zkVMs?**: Unlike circuit-based ZK systems (zk-SNARKs/STARKs), zkVMs allow developers to write standard software (in Rust) without needing to understand polynomial commitments.
- **Why Stellar?**: Stellar offers near-instant finality, incredibly low transaction fees, and a robust financial ecosystem (Anchors, USDC) that is perfectly suited for a high-throughput Compute Marketplace.

## 7. The Sadgi Standards
To create a thriving ecosystem, Sadgi introduces a family of open standards:
- **Sadgi Receipt Standard**: The common serialization format for cryptographic proofs on Stellar.
- **Sadgi Program Standard**: The boilerplate standard for writing compatible zkVM guest programs.
- **Sadgi Job Standard**: The on-chain representation of compute requests, including Priority and Redundancy metrics.
- **Sadgi Claim & Credential Standards**: Off-chain verifiable data schemas (e.g., KYC Age, Credit Scores) ensuring seamless interoperability across applications.

## 8. Compute Marketplace
The Compute Marketplace is Sadgi's decentralized scheduling engine.
- **Matching & Redundancy**: Jobs can be assigned to multiple Provers simultaneously. The fastest valid proof submitted claims the bounty, ensuring ultra-low latency for critical operations.
- **Capacity Advertising**: Provers advertise their CPU/RAM capabilities on-chain, allowing the Scheduler to route computationally heavy jobs efficiently.

## 9. Economics & Incentives
Sadgi relies entirely on **XLM** to align incentives.
- **Dynamic Staking**: Provers must lock an XLM stake that scales with their concurrent job load.
- **Treasury Model**: A small marketplace fee (e.g., 5%) from every fulfilled job is directed to the Protocol Treasury to fund ecosystem grants.
- **Progressive Discipline**: If a Prover misses a deadline, they suffer a Reputation penalty and temporary suspension, allowing them to recover from hardware crashes. Slashing (taking XLM) is strictly reserved for malicious cryptographic fraud.

## 10. Security & Threat Model
- **Fake Receipts (Cryptographic Attacks)**: Mitigated by the Soroban verifier, which mathematically rejects any invalid zkVM receipt. Malicious Provers lose their entire XLM stake instantly.
- **Spam (Marketplace Attacks)**: Mitigated by minimum XLM escrows required to queue a Job.
- **Front-Running**: Mitigated by assigning specific jobs to specific Provers. Even in redundant "fastest-wins" modes, only authorized Provers can claim the bounty.

## 11. Governance
Sadgi operates a tokenless, Progressive Decentralization model.
1. **Core Team**: A 3-of-5 Multisig to bootstrap the network.
2. **Foundation Stewardship**: Transitioning control to a non-profit entity for stability.
3. **Technical Council**: Expanding to verified node operators and ecosystem developers.
4. **Community Governance**: Fully decentralized, reputation-based stewardship using the Sadgi Improvement Proposal (SIP) lifecycle.

## 12. Developer Platform
The Sadgi Developer Platform goes beyond a simple SDK. It includes:
- **Starter Kits**: Pre-built templates for Identity, Compliance, and Data Processing.
- **API Explorer**: A robust Web UI to monitor job states and prover health.
- **Plugin System**: Allowing developers to easily swap proof backends (e.g., RISC Zero to SP1) as the zkVM landscape evolves.

## 13. Real Use Cases
- **Age Verification**: Proving a user is 18+ without revealing their birthdate to the dApp.
- **Academic Credentials**: Trustlessly verifying a university degree for employment smart contracts.
- **Payroll Verification**: Proving income requirements for DeFi lending protocols without leaking salary data.
- **AI Inference Verification**: Proving that an AI model produced a specific output based on a specific input, entirely off-chain.

## 14. Ecosystem Strategy
The Sadgi Flywheel:
As developers build more Reference Programs, more applications launch. This increases Proof Requests, driving demand for Provers. More Provers increases competition, lowering costs and latency, which in turn attracts even more developers to the Stellar ecosystem.

## 15. Roadmaps
- **Technical**: Finalize the RISC Zero verifier integration within Soroban and launch the Localnet Sandbox.
- **Product**: Launch the Next.js Explorer Dashboard, Developer Playground, and SDK v1.0.
- **Ecosystem**: Onboard the first major Stellar Wallet, Anchor, and DeFi protocol to utilize Sadgi verifiable credentials.

## 16. Vision 2030
By 2030, every Stellar wallet will verify zero-knowledge proofs natively. Every Soroban contract will have the ability to consume Sadgi receipts. Governments will issue verifiable credentials through Sadgi, banks will verify compliance privately, and AI systems will produce verifiable outputs. Sadgi will be the invisible, default trust layer powering the Stellar ecosystem.

## 17. Glossary
- **Receipt**: A cryptographic proof that a computation was executed correctly.
- **Journal**: The public outputs committed by the zkVM during execution.
- **Witness**: The private inputs used during computation that are never revealed.
- **Prover**: A node operator executing off-chain compute.

## 18. Call to Action
The verifiable future of Stellar is here. We invite you to build on Sadgi. Join the community, operate a Prover node, write Sadgi Improvement Proposals (SIPs), create new Reference Programs, and help us extend the protocol standards. The Trust Layer for Stellar applications is open.
