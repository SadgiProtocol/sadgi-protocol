# The Compute Marketplace

The Compute Marketplace is the Soroban-based scheduling and escrow engine that matches Developer demand with Prover supply.

## The Problem
Zero-Knowledge proving is computationally expensive. It cannot be run deterministically on a public blockchain ledger without bringing the network to a halt. Developers need a way to pay off-chain computers to generate these proofs, but they cannot trust off-chain computers to not steal their money.

## The Solution
The Sadgi Compute Marketplace acts as a trustless escrow and load balancer.

### 1. XLM Escrow
When a developer submits a `JobRequest`, they lock a bounty of XLM inside the Marketplace contract. The marketplace holds these funds securely.

### 2. Prover Staking
To participate in the marketplace, Prover Nodes must lock a stake of XLM. This stake acts as collateral. The maximum number of concurrent jobs a Prover can accept is proportional to their locked stake.

### 3. The Scheduler (Matching)
The Soroban contract acts as a decentralized Scheduler. When a `JobRequest` is received, the Marketplace assigns it to a Prover based on:
- **Capacity**: Does the Prover have enough staked collateral to accept the job?
- **Reputation**: Has this Prover successfully completed jobs on time in the past?

### 4. Settlement & Slashing
When the assigned Prover submits the `ProofReceipt`, the Marketplace contract immediately routes the receipt to the `SadgiVerifier`. 
- If `valid`, the XLM bounty is instantly unlocked and transferred to the Prover.
- If the Prover misses the deadline, they suffer a reputation penalty and a temporary timeout.
- If the Prover submits a cryptographically forged proof, they are **slashed** (their entire XLM stake is confiscated and burned).
