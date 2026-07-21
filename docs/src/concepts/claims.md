# Claims

A **claim** is a semantic, application-level assertion derived from one or more on-chain [receipts](./receipts.md). Where receipts are cryptographic artifacts, claims are human- and machine-readable statements about a subject.

## Claims vs. Receipts

| Aspect | Receipt | Claim |
|--------|---------|-------|
| Layer | Protocol | Application |
| Content | Raw proof outputs + prover metadata | Semantic assertion (e.g., "age ≥ 18") |
| Format | Soroban struct / CBOR | W3C Verifiable Credential (VC) JSON-LD |
| Consumer | Contracts | dApps, wallets, off-chain services |
| Issuer | Marketplace contract | Claim Issuer service or contract |

## Claim Structure

Sadgi claims follow the [W3C Verifiable Credentials Data Model 2.0](https://www.w3.org/TR/vc-data-model-2.0/):

```json
{
  "@context": ["https://www.w3.org/2018/credentials/v1", "https://sadgi.io/context/v1"],
  "type": ["VerifiableCredential", "SadgiClaim"],
  "issuer": "did:stellar:GABCDEF...",
  "issuanceDate": "2025-01-01T00:00:00Z",
  "credentialSubject": {
    "id": "did:stellar:GUSER...",
    "claim": "age_above_threshold",
    "threshold": 18,
    "receiptCID": "0xabc123...",
    "programId": "identity_v1"
  },
  "proof": {
    "type": "Ed25519Signature2020",
    "verificationMethod": "did:stellar:GABCDEF...#key-1",
    "proofValue": "z..."
  }
}
```

## How Claims Are Issued

```
User proves age off-chain (SP1 guest program)
           │
           ▼
  Groth16 SNARK submitted to Marketplace
           │
           ▼
  Receipt emitted on-chain (job_id, public_outputs: [age_above_18 = true])
           │
           ▼
  Claim Issuer reads receipt, verifies program_id == "identity_v1"
           │
           ▼
  Issues W3C VC binding the subject DID to the semantic claim
           │
           ▼
  User stores VC in their wallet / IPFS
```

## Claim Types in Sadgi

| Claim Type | Program | Semantic Assertion |
|------------|---------|-------------------|
| `AgeThreshold` | `identity_v1` | Subject's age ≥ threshold |
| `CreditScore` | `credit_v1` | Score in range [min, max] |
| `ReputationTier` | `reputation_v1` | On-chain activity tier (Bronze/Silver/Gold) |
| `GroupMembership` | `threshold_v1` | Member of a group (Merkle set membership) |
| `CredentialHolder` | `credential_v1` | Holds a valid W3C VC from a trusted issuer |

## Consuming Claims in a Contract

Contracts that need to enforce a claim can call the **Registry** to look up the program's VK hash and then verify the receipt directly:

```rust
// Check that a receipt for "identity_v1" exists and hasn't expired
let receipt = marketplace.get_receipt(&env, &job_id);
assert!(receipt.program_id == "identity_v1");
assert!(receipt.public_outputs.get(0) == Some(Val::from(true)));  // age_above_18
assert!(receipt.expires_at_ledger.map_or(true, |e| e > env.ledger().sequence()));
```

## Privacy Preservation

Claims are designed so that:
- The **on-chain receipt** only exposes public outputs (e.g., `age_above_18 = true`).
- The **private inputs** (actual birth date, raw credential data) never leave the prover's machine.
- The **claim VC** can be selectively disclosed—wallets show only the assertion, not the underlying data.

## Revocation

Claims derived from on-chain receipts are non-revocable by design (the receipt is immutable). For use cases requiring revocability, use expiring receipts (`receipt_ttl`) or implement a revocation registry pattern using a separate Soroban contract.

## See Also

- [Receipts](./receipts.md)
- [Identity Program](../programs/identity.md)
- [Credit Program](../programs/credit.md)
- [Receipt Standard](../architecture/receipt-standard.md)
