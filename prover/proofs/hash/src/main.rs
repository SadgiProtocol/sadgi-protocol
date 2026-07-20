#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // 1. Read caller contract ID (Standard IO)
    let caller_contract_id = sp1_zkvm::io::read::<[u8; 32]>();
    sp1_zkvm::io::commit(&caller_contract_id);

    // 2. Read the secret pre-image
    let pre_image = sp1_zkvm::io::read::<Vec<u8>>();

    // 3. Hash the pre-image (in a real SP1 app we'd use their accelerated SHA-256 crate,
    // but for this mockup we will just output a dummy hash or rely on simple logic to avoid bringing in sha2 dep)
    let is_valid = pre_image.len() > 0;

    // 4. Commit that we possess the pre-image
    sp1_zkvm::io::commit(&is_valid);
}
