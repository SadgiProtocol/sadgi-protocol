# Verifying Proofs

This guide covers how to verify that a Sadgi receipt is authentic—both on-chain (from a Soroban contract) and off-chain (from a TypeScript application).

## Verification Modes

| Mode | When to Use | Trust Assumption |
|------|------------|-----------------|
| **On-chain** | Smart contract gate-keeping access | Trustless (contract calls Verifier) |
| **Off-chain CID check** | UI display, API validation | Trusts Stellar RPC |
| **Full off-chain** | Auditing, archival | Self-contained, no RPC needed |

---

## On-Chain Verification (Soroban)

A Soroban contract can read a Marketplace receipt and enforce claims:

```rust
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

#[contract]
pub struct AccessGate;

#[contractimpl]
impl AccessGate {
    pub fn grant_access(env: Env, marketplace: Address, job_id: BytesN<32>) {
        // Read the receipt from the Marketplace contract
        let receipt: Receipt = env.invoke_contract(
            &marketplace,
            &symbol_short!("get_rcpt"),
            vec![&env, job_id.into_val(&env)],
        );

        // Enforce program and expiry
        assert!(receipt.program_id == "identity_v1", "Wrong program");
        if let Some(exp) = receipt.expires_at_ledger {
            assert!(exp > env.ledger().sequence(), "Receipt expired");
        }

        // Enforce claim (first public output = age_above_18)
        let age_ok: bool = receipt.public_outputs.get(0)
            .expect("Missing output")
            .into_val(&env);
        assert!(age_ok, "Age verification failed");

        // Grant access
        env.storage().persistent().set(&symbol_short!("access"), &receipt.prover);
    }
}
```

---

## Off-Chain Verification via SDK

### Check a Receipt by CID

```typescript
import { SadgiClient } from "@sadgi/sdk";

const client = new SadgiClient({ network: "testnet", publicKey, signer });

// Fetch receipt from on-chain storage
const receipt = await client.marketplace.getReceiptByCID("0xabc123...");
console.log("Program:", receipt.programId);
console.log("Outputs:", receipt.publicOutputs);
console.log("Valid:", !receipt.isExpired());
```

### Validate CID Authenticity

```typescript
import { verifyReceiptCID } from "@sadgi/sdk/utils";

// receipt is a plain JS object fetched from IPFS or your database
const isAuthentic = verifyReceiptCID(receipt, "0xabc123...");
// Returns true if SHA-256(CBOR(receipt)) === expectedCID
```

### Full Off-Chain Groth16 Verification

For auditing or archival use cases, you can re-verify the SNARK locally using the VK:

```typescript
import { verifyGroth16 } from "@sadgi/sdk/crypto";

const vk = await client.registry.getVK("identity_v1");
const isValid = await verifyGroth16({
  vk,
  proof: receipt.rawProof,           // 256 bytes
  publicInputs: receipt.rawPublicInputs,
});
console.log("Groth16 valid:", isValid);
```

This uses the BN254 pairing implementation bundled with the SDK (via `@aztec/bb.js`).

---

## Verifying W3C Verifiable Credentials (Claims)

If a user presents a Sadgi-issued VC, verify it with:

```typescript
import { verifyClaim } from "@sadgi/sdk/claims";

const result = await verifyClaim(credentialJWT, {
  trustedIssuers: ["did:stellar:GABCDEF..."],
  trustedPrograms: ["identity_v1", "credit_v1"],
  network: "testnet",
});

if (result.valid) {
  console.log("Claim type:", result.claimType);
  console.log("Subject:", result.subject);
  console.log("Receipt CID:", result.receiptCID);
}
```

`verifyClaim` performs:
1. JWT signature verification (Ed25519).
2. Issuer DID resolution against Stellar.
3. On-chain receipt lookup to confirm the underlying proof is still valid.

---

## Verification Checklist

When building a verifying contract or service, ensure you check all of the following:

- [ ] `receipt.program_id` matches your expected program.
- [ ] `receipt.vk_hash` matches `registry.get_vk_hash(program_id)`.
- [ ] `receipt.expires_at_ledger` is either null or in the future.
- [ ] `receipt.public_outputs` contain the expected semantic values.
- [ ] The receipt CID matches `SHA-256(CBOR(receipt))`.

## Common Mistakes

**Trusting `program_id` without checking `vk_hash`**: An attacker could register a malicious program under the same name. Always cross-reference the VK hash.

**Ignoring expiry**: Receipts with TTLs can be valid at submission time but expired by the time you check. Always evaluate `expires_at_ledger` against the current ledger sequence.

**Off-chain CID mismatch**: If you fetch a receipt from IPFS and the CID doesn't match, the data has been tampered with. Reject it.

## See Also

- [Requesting Proofs](./requesting.md)
- [Receipts](../concepts/receipts.md)
- [Receipt Standard](../architecture/receipt-standard.md)
