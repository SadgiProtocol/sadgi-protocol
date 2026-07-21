# Sadgi Protocol Architecture

The Sadgi Protocol bridges computationally expensive generic compute (via SP1 RISC-V zero-knowledge proofs) with the Stellar network through Soroban smart contracts.

## High-Level Flow

```mermaid
sequenceDiagram
    participant User (Client)
    participant Soroban Marketplace
    participant Prover Node (SP1)
    participant Oracle Network
    participant DAO Treasury
    
    User (Client)->>Soroban Marketplace: create_job(bounty, program_id)
    Soroban Marketplace-->>Soroban Marketplace: Lock XLM in Escrow
    Prover Node (SP1)->>Soroban Marketplace: Poll assigned jobs
    Prover Node (SP1)->>Prover Node (SP1): Execute ELF in ZKVM
    Prover Node (SP1)->>Prover Node (SP1): Generate STARK Proof
    Prover Node (SP1)->>Oracle Network: Submit STARK Proof
    Oracle Network-->>Oracle Network: Verify ZK Proof (Off-chain)
    Oracle Network->>Prover Node (SP1): Return Ed25519 Signed Public Values
    Prover Node (SP1)->>Soroban Marketplace: submit_proof(signed_values)
    Soroban Marketplace->>Soroban Marketplace: ed25519_verify()
    Soroban Marketplace->>Prover Node (SP1): Release 90% Bounty XLM
    Soroban Marketplace->>DAO Treasury: Route 10% Fee XLM
```

## Core Components

1. **Soroban Marketplace (`sadgi-marketplace`)**: Handles O(1) indexed job queues, manages escrow token locking, and coordinates deadlines. Uses `token::Client` to execute native transfers.
2. **Oracle Bridge (`sadgi-verifier`)**: Because native `bn254` pairing is not yet supported in Soroban, this uses `env.crypto().ed25519_verify()` to validate signatures provided by a trusted ZK oracle.
3. **Registry (`sadgi-registry`)**: Maintains verified Program ELFs and a **Trusted Issuers Merkle Root** for Verifiable Credentials DID.
4. **SP1 Node (`sadgi-prover-node`)**: Rust daemon integrating the `sp1-sdk` to execute RISC-V programs, generate cryptographic proofs, and wrap them with Oracle signatures before submission.
