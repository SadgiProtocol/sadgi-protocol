#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // 1. Read caller contract ID
    let caller_contract_id = sp1_zkvm::io::read::<[u8; 32]>();
    sp1_zkvm::io::commit(&caller_contract_id);

    // 2. Read public key, message, and signature
    let public_key_bytes = sp1_zkvm::io::read::<[u8; 32]>();
    let message = sp1_zkvm::io::read::<Vec<u8>>();
    let signature_bytes = sp1_zkvm::io::read::<Vec<u8>>();

    assert_eq!(
        signature_bytes.len(),
        64,
        "Signature must be exactly 64 bytes"
    );

    // 3. Verify signature using actual ed25519-dalek (hardware accelerated by SP1)
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    let public_key =
        VerifyingKey::from_bytes(&public_key_bytes).expect("Invalid public key format");

    let signature = Signature::from_slice(&signature_bytes).expect("Invalid signature format");

    public_key
        .verify(&message, &signature)
        .expect("Signature verification failed");

    // 4. Commit that this public key successfully signed the message
    sp1_zkvm::io::commit(&public_key_bytes);
}
