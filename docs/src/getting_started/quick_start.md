# Quick Start

Get your first ZK proof verified on Stellar in under 10 minutes.

## Prerequisites

- Node.js ≥ 20
- Rust toolchain (stable)
- `stellar-cli` installed and configured
- A funded Stellar Testnet account (use [Friendbot](https://friendbot.stellar.org))

## 1. Install the Sadgi SDK

```bash
npm install @sadgi/sdk @stellar/stellar-sdk @stellar/freighter-api
```

## 2. Connect Your Wallet

```typescript
import { getPublicKey, isConnected } from "@stellar/freighter-api";
import { SadgiClient } from "@sadgi/sdk";

if (!(await isConnected())) {
  throw new Error("Freighter wallet not found. Install it from freighter.app");
}

const publicKey = await getPublicKey();
const client = new SadgiClient({ network: "testnet", publicKey });
```

## 3. Browse Available ZK Programs

```typescript
const programs = await client.registry.listPrograms();
console.table(programs.map(p => ({ id: p.id, name: p.name, fee: p.fee })));
```

## 4. Submit a Proof Request

Pick a program (e.g., the built-in `hash_proof` program) and post a job:

```typescript
const jobId = await client.marketplace.postJob({
  programId: "hash_proof_v1",
  class: "Standard",       // min 10 XLM bounty
  bounty: "15",            // XLM
  inputCommitment: "0xabc123...", // SHA-256 of your private input
  deadline: Date.now() + 60 * 60 * 1000, // 1 hour
});

console.log("Job posted:", jobId);
```

## 5. Poll for the Verified Receipt

```typescript
const receipt = await client.marketplace.awaitReceipt(jobId, {
  pollIntervalMs: 5000,
  timeoutMs: 300_000,
});

console.log("Proof verified! Receipt CID:", receipt.cid);
console.log("Public outputs:", receipt.publicOutputs);
```

## 6. Use the Receipt in Your App

Once you have a receipt, any contract or off-chain verifier can trustlessly confirm that the computation was performed correctly:

```typescript
const isValid = await client.verifier.checkReceipt(receipt.cid);
console.log("Receipt valid on-chain:", isValid);
```

## What Just Happened?

1. You posted a **job** with a bounty locked in escrow.
2. A **prover** node picked up the job, ran your SP1 guest program, and generated a Groth16 SNARK.
3. The **Marketplace** contract called the **Verifier** contract, which checked the proof against the VK stored in the **Registry**.
4. On success, the bounty was released to the prover and a signed **receipt** was emitted on-chain.

## Next Steps

- Explore the [Local Environment](./local_environment.md) guide to run a prover node yourself.
- Read [Verifiable Computation](../concepts/verifiable_computation.md) for a deeper dive into how ZK proofs work in Sadgi.
- Browse [Example Programs](../programs/identity.md) to see what you can prove.
