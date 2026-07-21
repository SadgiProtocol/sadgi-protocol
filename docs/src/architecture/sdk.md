# SDK Architecture

The Sadgi SDK (`@sadgi/sdk`) is a TypeScript library that abstracts Soroban contract interactions, wallet integration, and proof lifecycle management into a clean, idiomatic API.

## Design Principles

1. **Wallet-agnostic**: Supports Freighter, Albedo, and raw keypair signing.
2. **Network-aware**: Switches between Testnet and Mainnet via a single config flag.
3. **Typed**: Full TypeScript types generated from contract XDR schemas.
4. **Reactive**: Exposes event streams for job state changes.

## Package Structure

```
@sadgi/sdk
├── src/
│   ├── client.ts          # SadgiClient entrypoint
│   ├── registry/          # Registry contract bindings
│   │   ├── index.ts
│   │   └── types.ts
│   ├── marketplace/       # Marketplace contract bindings
│   │   ├── index.ts
│   │   ├── job.ts
│   │   └── types.ts
│   ├── verifier/          # Verifier contract bindings
│   │   └── index.ts
│   ├── signing/           # Wallet adapters
│   │   ├── freighter.ts
│   │   └── keypair.ts
│   └── utils/
│       ├── cid.ts         # CID computation helpers
│       └── encoding.ts    # CBOR / XDR helpers
└── dist/                  # Compiled output (ESM + CJS)
```

## SadgiClient

The top-level `SadgiClient` class is the primary entry point:

```typescript
import { SadgiClient } from "@sadgi/sdk";

const client = new SadgiClient({
  network: "testnet",                     // "testnet" | "mainnet"
  rpcUrl: "https://soroban-testnet.stellar.org",
  publicKey: "GABC...",                   // signer public key
  signer: freighterSigner,               // WalletAdapter implementation
});

// Namespaced sub-clients
client.registry     // RegistryClient
client.marketplace  // MarketplaceClient
client.verifier     // VerifierClient
```

## Contract Bindings Generation

Bindings are generated automatically from the deployed contract's XDR interface using `stellar-cli`:

```bash
stellar contract bindings typescript \
  --contract-id <MARKETPLACE_CONTRACT_ID> \
  --output-dir src/marketplace/generated \
  --network testnet
```

This ensures types stay in sync with the on-chain contract interface.

## Wallet Adapters

```typescript
// Freighter (browser)
import { freighterSigner } from "@sadgi/sdk/signing/freighter";

// Raw keypair (server / CI)
import { keypairSigner } from "@sadgi/sdk/signing/keypair";
const signer = keypairSigner("SCECRETKEY...");
```

Both implement the `WalletAdapter` interface:

```typescript
interface WalletAdapter {
  signTransaction(xdr: string, network: Network): Promise<string>;
  publicKey(): Promise<string>;
}
```

## Job Lifecycle API

```typescript
// Post a job
const jobId = await client.marketplace.postJob({ ... });

// List open jobs (useful for provers)
const jobs = await client.marketplace.listJobs({ status: "Open", limit: 20 });

// Stream job events
const unsubscribe = client.marketplace.onJobEvent(jobId, (event) => {
  console.log(event.type, event.data);
});

// Await receipt
const receipt = await client.marketplace.awaitReceipt(jobId, {
  pollIntervalMs: 3000,
  timeoutMs: 120_000,
});
```

## Error Handling

All SDK calls throw typed `SadgiError` instances:

```typescript
import { SadgiError, SadgiErrorCode } from "@sadgi/sdk";

try {
  await client.marketplace.postJob({ ... });
} catch (err) {
  if (err instanceof SadgiError) {
    switch (err.code) {
      case SadgiErrorCode.InsufficientBounty:
        console.error("Bounty below minimum for job class");
        break;
      case SadgiErrorCode.ProgramNotFound:
        console.error("Program ID not registered");
        break;
    }
  }
}
```

## See Also

- [Requesting Proofs](../sdk/requesting.md)
- [Verifying Proofs](../sdk/verifying.md)
- [Contracts Architecture](./contracts.md)
