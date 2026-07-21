# Credit Program (`credit_v1`)

The Credit Program enables privacy-preserving credit scoring. Users can prove their credit score lies within a given range—or exceeds a minimum threshold—without revealing the exact score or underlying financial data.

## Use Cases

- **DeFi lending**: Prove creditworthiness to unlock undercollateralized loans.
- **Protocol fee tiers**: Prove score ≥ 700 to qualify for reduced trading fees.
- **Insurance pricing**: Prove risk tier without disclosing financial history.
- **Employer checks**: Prove financial reliability for sensitive roles.

## Program ID

```
credit_v1
```

## Guest Program Inputs

| Input | Type | Visibility | Description |
|-------|------|-----------|-------------|
| `score_report_bytes` | `Vec<u8>` | Private | Signed credit report (JSON, from trusted bureau) |
| `bureau_pubkey` | `[u8; 32]` | Public | Ed25519 public key of the credit bureau |
| `check_type` | `u8` | Public | Comparison to perform (see table) |
| `threshold_min` | `u32` | Public | Minimum score (inclusive) |
| `threshold_max` | `u32` | Public | Maximum score (inclusive) |
| `report_timestamp_window` | `u64` | Public | Max age of report (seconds) |
| `current_timestamp` | `u64` | Public | Current Unix timestamp |

## Public Outputs

| Index | Type | Description |
|-------|------|-------------|
| `0` | `bool` | `true` if the score satisfies the condition |
| `1` | `[u8; 32]` | Report nullifier (prevents reuse) |
| `2` | `[u8; 32]` | Bureau public key hash |
| `3` | `u64` | Report timestamp (proves freshness) |

## Check Type Codes

| Code | Name | Condition |
|------|------|-----------|
| `0` | `AboveMin` | `score >= threshold_min` |
| `1` | `InRange` | `threshold_min <= score <= threshold_max` |
| `2` | `BelowMax` | `score <= threshold_max` |

## How It Works

Inside the SP1 zkVM:

1. **Deserialize report**: Parse the credit bureau's signed JSON report.
2. **Verify bureau signature**: Ed25519 signature check against `bureau_pubkey`.
3. **Check freshness**: Assert `report.timestamp > current_timestamp - report_timestamp_window`.
4. **Extract score**: Read `report.score` (integer, 300–850 range).
5. **Apply check**: Evaluate the selected `check_type` against thresholds.
6. **Compute nullifier**: `Poseidon(report_id || subject_pubkey)`.
7. **Commit outputs**: Expose `[satisfied, nullifier, bureau_hash, report_timestamp]`.

## Integration Example: DeFi Lending

```typescript
// Borrower side
const report = await fetchCreditReport(); // from your bureau partner
const commitment = await computeInputCommitment(report);
const encryptedInput = await encryptForProver(report, proverPublicKey);
const inputCID = await pinToIPFS(encryptedInput);

const jobId = await client.marketplace.postJob({
  programId: "credit_v1",
  class: "Priority",
  bounty: "55",
  inputCommitment: commitment,
  inputCID,
  publicInputs: {
    bureau_pubkey: EQUIFAX_TESTNET_KEY,
    check_type: 0,          // AboveMin
    threshold_min: 650,
    threshold_max: 850,     // ignored for AboveMin
    report_timestamp_window: 30 * 24 * 3600,  // 30 days
    current_timestamp: Math.floor(Date.now() / 1000),
  },
});

// Lender / contract side
const receipt = await client.marketplace.awaitReceipt(jobId);
const [creditworthy, nullifier, bureauHash, reportTs] = receipt.publicOutputs;

if (creditworthy && bureauHash === EQUIFAX_HASH) {
  await lendingContract.approveApplication(borrowerAddress, jobId);
}
```

## On-Chain Enforcement (Soroban)

```rust
pub fn approve_loan(env: Env, marketplace: Address, job_id: BytesN<32>, amount: i128) {
    let receipt: Receipt = env.invoke_contract(&marketplace, &symbol_short!("get_rcpt"), ...);

    assert!(receipt.program_id == "credit_v1");

    // Check bureau is trusted
    let bureau_hash: BytesN<32> = receipt.public_outputs.get(2).unwrap().into_val(&env);
    assert!(is_trusted_bureau(&env, &bureau_hash), "Unknown bureau");

    // Check score condition met
    let satisfied: bool = receipt.public_outputs.get(0).unwrap().into_val(&env);
    assert!(satisfied, "Credit score insufficient");

    // Check nullifier not already used
    let nullifier: BytesN<32> = receipt.public_outputs.get(1).unwrap().into_val(&env);
    assert!(!nullifier_used(&env, &nullifier), "Proof already consumed");
    mark_nullifier_used(&env, &nullifier);

    // Disburse loan...
}
```

## Freshness Policy

Credit reports older than `report_timestamp_window` will cause the guest program to panic, making the proof invalid. Recommended windows:

| Use Case | Max Age |
|----------|---------|
| Mortgage / large loan | 7 days |
| Personal loan | 30 days |
| Protocol fee tier | 90 days |

## Supported Bureaus (Testnet)

| Bureau | Program Name | Public Key Hash |
|--------|-------------|----------------|
| Sadgi Demo Bureau | `sadgi_demo_bureau_v1` | `0xd3a1...` |

## See Also

- [Identity Program](./identity.md)
- [Claims](../concepts/claims.md)
- [Verifying Proofs](../sdk/verifying.md)
