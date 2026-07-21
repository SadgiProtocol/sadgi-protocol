# Identity Program (`identity_v1`)

The Identity Program allows users to prove attributes about their identity—such as age, nationality, or credential possession—without revealing the underlying personal data.

## Use Cases

- **Age gating**: Prove you are 18+ to access DeFi protocols without KYC.
- **Citizenship**: Prove residency in a specific country without exposing your passport.
- **Credential verification**: Prove you hold a university degree or professional certification.
- **Sybil resistance**: Prove uniqueness (one person = one account) without a central authority.

## Program ID

```
identity_v1
```

## Guest Program Inputs

| Input | Type | Visibility | Description |
|-------|------|-----------|-------------|
| `credential_bytes` | `Vec<u8>` | Private | Raw W3C VC JSON-LD credential bytes |
| `issuer_pubkey` | `[u8; 32]` | Public | Ed25519 public key of the trusted issuer |
| `claim_type` | `u8` | Public | Claim to prove (see table below) |
| `threshold` | `u32` | Public | Numeric threshold (e.g., age = 18) |
| `current_timestamp` | `u64` | Public | Unix timestamp for expiry checks |

## Public Outputs

| Index | Type | Description |
|-------|------|-------------|
| `0` | `bool` | `true` if the claim is satisfied |
| `1` | `[u8; 32]` | Nullifier (prevents double-use of same credential) |
| `2` | `[u8; 32]` | Issuer public key hash |

## Claim Type Codes

| Code | Name | Description |
|------|------|-------------|
| `0` | `AgeAbove` | Subject's age ≥ threshold |
| `1` | `AgeBelow` | Subject's age ≤ threshold |
| `2` | `CountryCode` | Subject's country matches threshold (ISO 3166 numeric) |
| `3` | `CredentialType` | Subject holds a VC of a specific type |
| `4` | `Uniqueness` | Subject is a unique individual (Merkle set membership) |

## How It Works

The SP1 guest program performs the following steps inside the zkVM:

1. **Parse the VC**: Deserialize the JSON-LD credential and verify its Ed25519 signature against `issuer_pubkey`.
2. **Check expiry**: Assert `credential.expirationDate > current_timestamp`.
3. **Extract the attribute**: Read the relevant field (e.g., `birthDate`) from `credentialSubject`.
4. **Evaluate the claim**: Apply the threshold check (e.g., `age >= 18`).
5. **Compute nullifier**: `nullifier = Poseidon(credential_id || prover_sk)` — ties the proof to one use.
6. **Commit public outputs**: Expose `[claim_satisfied, nullifier, issuer_hash]` to the verifier.

## Requesting an Identity Proof

```typescript
import { computeInputCommitment, encryptForProver } from "@sadgi/sdk/utils";

const vcBytes = await fetchMyCredential(); // your W3C VC as Uint8Array
const commitment = await computeInputCommitment(vcBytes);

// Encrypt VC for the prover
const encryptedInput = await encryptForProver(vcBytes, proverPublicKey);
const inputCID = await pinToIPFS(encryptedInput);

const jobId = await client.marketplace.postJob({
  programId: "identity_v1",
  class: "Standard",
  bounty: "12",
  inputCommitment: commitment,
  inputCID,
  publicInputs: {
    issuer_pubkey: TRUSTED_ISSUER_PUBKEY,
    claim_type: 0,      // AgeAbove
    threshold: 18,
    current_timestamp: Math.floor(Date.now() / 1000),
  },
});
```

## Verifying the Receipt

```typescript
const receipt = await client.marketplace.awaitReceipt(jobId);
const [claimSatisfied, nullifier, issuerHash] = receipt.publicOutputs;

if (claimSatisfied) {
  console.log("Age verified! Nullifier:", nullifier);
  // Store nullifier to prevent replay attacks
}
```

## Security Notes

- **Nullifiers**: Always store used nullifiers and reject duplicate claims with the same nullifier.
- **Issuer trust**: Only accept receipts where `issuer_hash` corresponds to a trusted issuer's key.
- **VC freshness**: Set a reasonable `receipt_ttl` to prevent stale identity claims being reused.

## Trusted Issuers (Testnet)

| Name | Public Key |
|------|-----------|
| Sadgi Demo Issuer | `GBXYZ...` |
| OpenCerts Testnet | `GCABC...` |

## See Also

- [Claims](../concepts/claims.md)
- [Credit Program](./credit.md)
- [Requesting Proofs](../sdk/requesting.md)
