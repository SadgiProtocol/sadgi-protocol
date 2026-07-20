# The Sadgi Program Standard (SIP-3)

To ensure interoperability between the Sadgi Prover Node and arbitrary developer programs, all zkVM guest programs must adhere to the Sadgi Program Standard.

## Standard IO
Because zkVM guest programs execute in an isolated environment, they receive inputs via the Host (the Prover Node) and write outputs to a public Journal.

A compliant Sadgi Program MUST:
1. **Read the `caller_contract_id`** as the first input from the Host.
2. **Commit the `caller_contract_id`** as the first 32 bytes of the Journal.

## Example (RISC Zero)

```rust
use risc0_zkvm::guest::env;

fn main() {
    // 1. Read the Soroban Contract ID requesting this computation.
    let caller_contract_id: [u8; 32] = env::read();
    
    // 2. Commit it to the journal immediately to prevent Replay Attacks.
    env::commit_slice(&caller_contract_id);
    
    // 3. Perform arbitrary heavy computation...
    let user_data: Vec<u8> = env::read();
    let is_valid = verify_heavy_cryptography(user_data);
    
    // 4. Commit the business logic results.
    env::commit(&is_valid);
}
```

By strictly adhering to this standard, the Soroban `SadgiReceipt` Verifier can securely map the cryptographic proof back to the exact Smart Contract that requested it.
