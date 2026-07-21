# Receipt Standard

The Sadgi Receipt Standard defines the canonical encoding, schema, and lifecycle rules for receipts emitted by the Marketplace contract. Adhering to this standard ensures interoperability between contracts, SDKs, and off-chain indexers.

## Motivation

Without a standard, each dApp would interpret proof outputs differently, leading to fragmented ecosystems. The receipt standard provides a stable, versioned schema that any Soroban contract can consume without tight coupling to a specific program.

## Canonical Encoding

All receipts MUST be CBOR-encoded (RFC 7049) before hashing. The canonical field order is:

```
[
  job_id,           ; bytes(32)
  program_id,       ; tstr
  vk_hash,          ; bytes(32)
  public_outputs,   ; array of any
  prover,           ; bytes(32)  (Stellar public key)
  verified_at_ledger, ; uint
  expires_at_ledger   ; uint or null
]
```

The CID is `SHA-256( canonical_cbor(receipt) )` encoded as a 32-byte `BytesN<32>`.

## Storage Layout

Receipts are stored in Soroban persistent storage under the following key pattern:

```
DataKey::Receipt(BytesN<32>)   →   Receipt struct
DataKey::ReceiptIndex(Address) →   Vec<BytesN<32>>  (prover's receipt list)
```

The `ReceiptIndex` facilitates enumeration of all receipts produced by a given prover.

## Event Schema

On successful verification, the Marketplace emits a `ProofVerified` event:

```rust
env.events().publish(
    (symbol_short!("pv"), job_id.clone()),
    (program_id.clone(), cid.clone(), prover.clone()),
);
```

Indexers MUST subscribe to this event topic to track all verified proofs.

## Versioning

The standard is versioned via the `program_id` field, which encodes both the program name and version: `<name>_v<major>`. Breaking changes to a program's public output schema require incrementing the major version and registering a new VK in the Registry.

| Version | Change |
|---------|--------|
| `_v1` | Initial release |
| `_v2` | Adds a `metadata_hash` field to public outputs |

Consumers SHOULD check the `program_id` suffix before parsing `public_outputs`.

## Interoperability Rules

1. **Cross-contract reads**: Any contract may call `marketplace.get_receipt(job_id)` to read a receipt.
2. **Expiry enforcement**: Consumers MUST check `expires_at_ledger` and reject expired receipts.
3. **CID pinning**: High-value receipts SHOULD be pinned to IPFS/Arweave before Soroban's TTL eviction.
4. **No mutation**: Receipts are immutable after creation. Do not attempt to overwrite a receipt.
5. **Program whitelisting**: Contracts that consume receipts SHOULD maintain a whitelist of trusted `program_id` values.

## Reference Implementation

The canonical receipt type is defined in the `sadgi-types` crate:

```toml
# Cargo.toml
[dependencies]
sadgi-types = { git = "https://github.com/sadgi-protocol/sadgi-core", package = "sadgi-types" }
```

```rust
use sadgi_types::Receipt;

// Verify CID matches
let computed_cid = Receipt::compute_cid(&env, &receipt);
assert_eq!(computed_cid, expected_cid);
```

## Security Considerations

- The receipt only proves that a Groth16 SNARK was valid for the registered VK. It does not prove the correctness of the program's source code—review the guest program carefully.
- If a trusted setup is compromised, an adversary could forge proofs. Sadgi inherits SP1's universal SRS security assumptions.
- Always verify `vk_hash` against the Registry, not just `program_id`, to prevent substitution attacks.

## See Also

- [Receipts](../concepts/receipts.md)
- [Contracts Architecture](./contracts.md)
- [Verifier Contract](./contracts.md#verifier)
