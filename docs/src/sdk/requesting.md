# Requesting Proofs

This guide covers how to use the Sadgi SDK to post proof requests (jobs) to the Marketplace contract and handle the full request lifecycle.

## Overview

Requesting a proof involves three steps:
1. Choosing a registered ZK program.
2. Constructing an input commitment (a hash of your private inputs).
3. Posting the job and waiting for a receipt.

## Step 1 — Choose a Program

List all registered programs to find one that suits your use case:

```typescript
import { SadgiClient } from "@sadgi/sdk";

const client = new SadgiClient({ network: "testnet", publicKey, signer });
const programs = await client.registry.listPrograms();

// [{ id: "hash_proof_v1", name: "SHA-256 Hash Proof", vkHash: "0x..." }, ...]
```

## Step 2 — Compute an Input Commitment

Your private inputs must never be sent on-chain. Instead, hash them and submit only the commitment:

```typescript
import { computeInputCommitment } from "@sadgi/sdk/utils";

// Example: proving you know the preimage of a SHA-256 hash
const privateInput = new TextEncoder().encode("my secret data");
const commitment = await computeInputCommitment(privateInput);
// Returns a 32-byte hex string: "0xabc123..."
```

The commitment is a SHA-256 hash of your inputs. The prover will receive the actual inputs via a secure off-chain channel (e.g., encrypted IPFS, direct P2P).

## Step 3 — Post the Job

```typescript
const jobId = await client.marketplace.postJob({
  programId: "hash_proof_v1",
  class: "Standard",           // "Standard" | "Priority" | "Bulk" | "AI"
  bounty: "15",                // XLM amount as string
  inputCommitment: commitment, // 32-byte hex
  deadlineOffset: 3600,        // seconds from now
  // Optional: encrypted input payload for provers
  inputCID: "ipfs://Qm...",
});

console.log("Job ID:", jobId);
```

### Job Classes and Minimum Bounties

| Class | Min Bounty | Expected Turnaround |
|-------|-----------|-------------------|
| `Bulk` | 5 XLM | Hours |
| `Standard` | 10 XLM | Minutes |
| `Priority` | 50 XLM | < 60 seconds |
| `AI` | 200 XLM | 5–30 minutes |

If `bounty` is below the class minimum, the transaction will be rejected with `SadgiErrorCode.InsufficientBounty`.

## Step 4 — Monitor Job Status

Poll for status changes or subscribe to events:

```typescript
// Polling approach
const job = await client.marketplace.getJob(jobId);
console.log(job.status); // "Posted" | "Claimed" | "Verified" | "Cancelled"

// Event stream approach
const unsubscribe = client.marketplace.onJobEvent(jobId, (event) => {
  if (event.type === "JobClaimed") {
    console.log("Prover claimed your job:", event.data.prover);
  }
  if (event.type === "ProofVerified") {
    console.log("Proof verified! Receipt:", event.data.receiptCID);
    unsubscribe();
  }
});
```

## Step 5 — Retrieve the Receipt

```typescript
const receipt = await client.marketplace.awaitReceipt(jobId, {
  pollIntervalMs: 5000,
  timeoutMs: 300_000,  // 5 minutes
});

console.log("Program:", receipt.programId);
console.log("Outputs:", receipt.publicOutputs);
console.log("CID:", receipt.cid);
console.log("Verified at ledger:", receipt.verifiedAtLedger);
```

## Cancelling a Job

If a job is not claimed before its deadline, you can cancel it and reclaim the bounty:

```typescript
await client.marketplace.cancelJob(jobId);
// Bounty is returned to your address
```

> **Note**: You cannot cancel a job that has already been claimed by a prover. Wait for the deadline to expire first.

## Providing Inputs to the Prover

Provers need your private inputs to generate the proof. Sadgi supports two delivery methods:

### IPFS (Recommended)
Encrypt inputs with the prover's public key, pin to IPFS, and set `inputCID` when posting the job.

### Direct Channel
For Priority jobs, use Sadgi's end-to-end encrypted relay:

```typescript
await client.relay.sendInput(jobId, proverPublicKey, encryptedInputBuffer);
```

## Error Handling

```typescript
import { SadgiError, SadgiErrorCode } from "@sadgi/sdk";

try {
  const jobId = await client.marketplace.postJob({ ... });
} catch (err) {
  if (err instanceof SadgiError) {
    if (err.code === SadgiErrorCode.ProgramNotFound) {
      console.error("Check your programId against client.registry.listPrograms()");
    }
  }
}
```

## See Also

- [Verifying Proofs](./verifying.md)
- [SDK Architecture](../architecture/sdk.md)
- [Quick Start](../getting_started/quick_start.md)
