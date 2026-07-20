# The Sadgi Program Standard (SIP-3)

To ensure interoperability between the Sadgi Prover Node and arbitrary developer programs, all zkVM guest programs must adhere to the Sadgi Program Standard.

## Standard IO
Because zkVM guest programs execute in an isolated environment, they receive inputs via the Host (the Prover Node) and write outputs to a public Journal.

A compliant Sadgi Program MUST:
1. **Read the `caller_contract_id`** as the first input from the Host.
2. **Commit the `caller_contract_id`** as the first 32 bytes of the Journal.

## Example (SP1)

```rust
#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // 1. Read the Soroban Contract ID requesting this computation.
    let caller_contract_id = sp1_zkvm::io::read::<[u8; 32]>();
    
    // 2. Commit it to the journal immediately to prevent Replay Attacks.
    sp1_zkvm::io::commit(&caller_contract_id);
    
    // 3. Perform arbitrary heavy computation...
    let user_data = sp1_zkvm::io::read::<Vec<u8>>();
    let is_valid = user_data.len() > 0; // Example heavy cryptography
    
    // 4. Commit the business logic results.
    sp1_zkvm::io::commit(&is_valid);
}
```

By strictly adhering to this standard, the Soroban `ProofReceipt` Verifier can securely map the cryptographic proof back to the exact Smart Contract that requested it.
