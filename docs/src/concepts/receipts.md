# Receipts

A **receipt** is the on-chain artifact produced when the Marketplace contract successfully verifies a ZK proof. It is the primary unit of trust in the Sadgi Protocol.

## What Is a Receipt?

After a prover submits a valid Groth16 SNARK for a job, the Marketplace contract:

1. Calls `Verifier.verify(vk, proof, public_inputs)`.
2. On success, persists a receipt record and emits a `ProofVerified` event.
3. Releases the bounty to the prover's address.

The receipt binds together:
- The **job ID** that was proven.
- The **program ID** (and its VK hash) that was executed.
- The **public outputs** of the computation.
- The **prover address** that submitted the proof.
- The **ledger sequence** at which the proof was verified.

## Receipt Data Structure

```rust
pub struct Receipt {
    pub job_id: BytesN<32>,
    pub program_id: String,
    pub vk_hash: BytesN<32>,
    pub public_outputs: Vec<Val>,
    pub prover: Address,
    pub verified_at_ledger: u32,
    pub expires_at_ledger: Option<u32>,
}
```

Receipts are stored in Soroban's **persistent storage** under the key `Receipt(job_id)` and are therefore durable across ledger closings (subject to rent payments).

## Receipt Lifecycle

```
Job Posted ──► Proof Submitted ──► Verified ──► Receipt Emitted
                                                     │
                             ┌───────────────────────┤
                             │                       │
                      consumed by dApp         referenced by
                      (e.g., grant access)     another contract
```

A receipt is **immutable** after creation. Any contract can read it:

```rust
// Soroban pseudocode
let receipt: Receipt = env.invoke_contract(
    &marketplace_addr,
    &symbol_short!("get_rcpt"),
    vec![&env, job_id.into_val(&env)],
);
assert!(receipt.program_id == "identity_v1");
```

## Content Addressing (CID)

Each receipt is also content-addressed using a SHA-256 CID derived from the canonical CBOR encoding of the struct. The CID is emitted in the `ProofVerified` event and can be used to pin the receipt to IPFS or Arweave for long-term storage beyond Soroban's rent window.

```
CID = SHA256( CBOR( Receipt ) )
```

## Receipt Expiry

Receipts can have an optional `expires_at_ledger`. When a job is posted with a `receipt_ttl` parameter, the Marketplace sets `expires_at_ledger = current_ledger + receipt_ttl`. After expiry:

- The on-chain storage entry is eligible for eviction.
- The CID remains valid for off-chain verification if the data is pinned.
- Contracts that rely on the receipt should check expiry before consuming it.

## Verifying a Receipt Off-Chain

The SDK provides a helper to verify receipt authenticity without an RPC call:

```typescript
import { verifyReceiptCID } from "@sadgi/sdk";

const isAuthentic = verifyReceiptCID(receiptData, expectedCID);
// Returns true if the CID matches the canonical encoding of the receipt fields.
```

## Relationship to Claims

A receipt is a **raw cryptographic artifact**. A [claim](./claims.md) is a higher-level semantic assertion (e.g., "this user is over 18") derived from one or more receipts. Claims are what application developers typically interact with.

## See Also

- [Claims](./claims.md)
- [Verifiable Computation](./verifiable_computation.md)
- [Marketplace Contract](../architecture/contracts.md#marketplace)
