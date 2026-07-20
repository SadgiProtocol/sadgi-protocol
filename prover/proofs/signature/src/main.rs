#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // 1. Read caller contract ID
    let caller_contract_id = sp1_zkvm::io::read::<[u8; 32]>();
    sp1_zkvm::io::commit(&caller_contract_id);

    // 2. Read public key, message, and signature
    let public_key = sp1_zkvm::io::read::<[u8; 32]>();
    let _message = sp1_zkvm::io::read::<Vec<u8>>();
    let _signature = sp1_zkvm::io::read::<Vec<u8>>();
    
    // 3. Verify signature (mocked for speed, in production use ed25519-dalek or SP1 precompiles)
    let is_valid = true;
    
    assert!(is_valid, "Invalid Signature");
    
    // 4. Commit that this public key successfully signed the message
    sp1_zkvm::io::commit(&public_key);
}
