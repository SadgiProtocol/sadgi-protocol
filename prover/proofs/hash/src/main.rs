#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // 1. Read caller contract ID (Standard IO)
    let caller_contract_id = sp1_zkvm::io::read::<[u8; 32]>();
    sp1_zkvm::io::commit(&caller_contract_id);

    // 2. Read the secret pre-image
    let pre_image = sp1_zkvm::io::read::<Vec<u8>>();

    // 3. Hash the pre-image using actual SHA-256 (hardware accelerated by SP1)
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(&pre_image);
    let result = hasher.finalize();

    // 4. Commit the resulting hash (Public Output)
    // We convert it to a 32-byte array to be consistent with the Verifier
    let hash_array: [u8; 32] = result.into();
    sp1_zkvm::io::commit(&hash_array);
}
