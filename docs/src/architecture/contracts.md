# Contracts Architecture

The Sadgi Protocol is implemented as three composable Soroban smart contracts deployed on Stellar. Each contract has a distinct responsibility and a well-defined interface.

## Contract Overview

```
┌─────────────────────────────────────────────────────────┐
│                     Marketplace                          │
│  Job lifecycle, escrow, bounty distribution              │
│  Calls ──► Registry (VK lookup)                         │
│  Calls ──► Verifier (proof check)                       │
└───────────────────────┬─────────────────────────────────┘
                        │
          ┌─────────────┴─────────────┐
          ▼                           ▼
   ┌─────────────┐           ┌─────────────────┐
   │  Registry   │           │    Verifier      │
   │  VK storage │           │  Groth16 verify  │
   └─────────────┘           └─────────────────┘
```

---

## Registry

**Purpose**: Stores Verification Keys (VKs) for all registered ZK programs. Any prover or consumer can look up a VK by program ID.

### Key Interfaces

```rust
pub trait RegistryTrait {
    /// Admin-only: register a new ZK program.
    fn register_program(env: Env, admin: Address, program_id: String, vk: Bytes, name: String) -> BytesN<32>;

    /// Retrieve the VK for a program (returns None if not found).
    fn get_vk(env: Env, program_id: String) -> Option<Bytes>;

    /// Retrieve the VK hash for a program.
    fn get_vk_hash(env: Env, program_id: String) -> Option<BytesN<32>>;

    /// List all registered program IDs.
    fn list_programs(env: Env) -> Vec<String>;
}
```

### Storage

| Key | Value | TTL |
|-----|-------|-----|
| `Program(id)` | `ProgramEntry { vk, name, vk_hash, registered_at }` | Persistent |
| `ProgramList` | `Vec<String>` | Persistent |

---

## Verifier

**Purpose**: Exposes a single `verify` function that checks a Groth16 SNARK against a provided VK and public inputs. Stateless—holds no storage.

### Key Interfaces

```rust
pub trait VerifierTrait {
    /// Verify a Groth16 proof.
    /// Returns true if valid, panics with VerificationFailed if invalid.
    fn verify(
        env: Env,
        vk: Bytes,
        proof: Bytes,           // 3 BN254 G1/G2 points, encoded
        public_inputs: Vec<Bytes>,
    ) -> bool;
}
```

### Proof Encoding

Proofs are encoded as:
```
proof[0..64]   = A  (G1 point, uncompressed)
proof[64..192] = B  (G2 point, uncompressed)
proof[192..256] = C (G1 point, uncompressed)
```
Total: **256 bytes** per proof.

Public inputs are encoded as 32-byte big-endian field elements (BN254 scalar field).

---

## Marketplace

**Purpose**: Manages the full job lifecycle—from posting through proof submission to bounty payment. Holds XLM in escrow during job execution.

### Job Class Minimums

| Class | Min Bounty | Use Case |
|-------|-----------|---------|
| `Standard` | 10 XLM | General computation |
| `Priority` | 50 XLM | Fast turnaround SLA |
| `Bulk` | 5 XLM | High-volume, low-priority batch |
| `AI` | 200 XLM | Large ML inference proofs |

### Key Interfaces

```rust
pub trait MarketplaceTrait {
    fn initialize(env: Env, admin: Address, registry: Address, verifier: Address);

    fn post_job(env: Env, poster: Address, program_id: String, class: JobClass,
                bounty: i128, input_commitment: BytesN<32>, deadline_ledger: u32) -> BytesN<32>;

    fn claim_job(env: Env, prover: Address, job_id: BytesN<32>);

    fn submit_proof(env: Env, prover: Address, job_id: BytesN<32>,
                    proof: Bytes, public_inputs: Vec<Bytes>);

    fn cancel_job(env: Env, poster: Address, job_id: BytesN<32>);

    fn get_job(env: Env, job_id: BytesN<32>) -> Job;

    fn get_receipt(env: Env, job_id: BytesN<32>) -> Receipt;

    fn list_jobs(env: Env, status: Option<JobStatus>, limit: u32) -> Vec<Job>;
}
```

### Job State Machine

```
       ┌──────────┐
       │  Posted  │◄──── postJob()
       └────┬─────┘
            │ claimJob()
       ┌────▼─────┐
       │ Claimed  │
       └────┬─────┘
            │ submitProof() [valid]     deadline exceeded
       ┌────▼──────┐             ┌───────────────┐
       │ Verified  │             │   Cancelled   │◄── cancelJob()
       └───────────┘             └───────────────┘
```

### Escrow Mechanism

- On `postJob`: `bounty` XLM is transferred from poster to Marketplace's contract address.
- On `submit_proof` (valid): `bounty` XLM is transferred to prover; receipt is persisted.
- On `cancel_job`: `bounty` XLM is returned to poster (only allowed after deadline or if unclaimed).

## Deployment

All three contracts are deployed via `stellar-cli` and their addresses are wired together during `Marketplace.initialize()`. See [Local Environment](../getting_started/local_environment.md) for deployment steps.
