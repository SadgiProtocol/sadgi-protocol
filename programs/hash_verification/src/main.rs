#![no_main]

use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};

risc0_zkvm::guest::entry!(main);

fn main() {
    // 1. Read the secret pre-image from the private host input
    let pre_image: String = env::read();

    // 2. Hash the secret pre-image inside the zkVM
    let mut hasher = Sha256::new();
    hasher.update(pre_image.as_bytes());
    let result = hasher.finalize();

    // 3. Commit ONLY the public hash result to the Journal.
    // The prover proves they know the pre_image that hashes to this output,
    // without revealing the pre_image itself!
    env::commit(&result.as_slice());
}
