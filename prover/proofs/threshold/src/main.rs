#![no_main]
#![allow(dead_code)]
sp1_zkvm::entrypoint!(main);

extern crate alloc;
use alloc::vec::Vec;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sadgi_types::threshold::{ThresholdPayload, ThresholdVerificationOutput};
use sha2::{Digest, Sha256};

fn verify_merkle_proof(
    root: &[u8; 32],
    leaf: &[u8; 32],
    path: &[[u8; 32]],
    indices: &[bool],
) -> bool {
    let mut current_hash = *leaf;
    for (i, sibling) in path.iter().enumerate() {
        let mut hasher = Sha256::new();
        if indices[i] {
            hasher.update(sibling);
            hasher.update(current_hash);
        } else {
            hasher.update(current_hash);
            hasher.update(sibling);
        }
        current_hash.copy_from_slice(&hasher.finalize());
    }
    &current_hash == root
}

pub fn main() {
    // 1. Read Public Inputs (Policies)
    let trusted_issuers_root = sp1_zkvm::io::read::<[u8; 32]>();
    let target_threshold = sp1_zkvm::io::read::<i32>();

    // 2. Read Private Inputs (Witnesses)
    let payload_bytes = sp1_zkvm::io::read::<Vec<u8>>();
    let signature_vec = sp1_zkvm::io::read::<Vec<u8>>();
    let signature_bytes: [u8; 64] = signature_vec.try_into().expect("Invalid signature length");
    let issuer_pubkey_bytes = sp1_zkvm::io::read::<[u8; 32]>();
    let merkle_path = sp1_zkvm::io::read::<Vec<[u8; 32]>>();
    let merkle_indices = sp1_zkvm::io::read::<Vec<bool>>();

    // 3. Verify Merkle Proof (Authenticate the Issuer)
    let mut hasher = Sha256::new();
    hasher.update(issuer_pubkey_bytes);
    let leaf_hash: [u8; 32] = hasher.finalize().into();

    assert!(
        verify_merkle_proof(
            &trusted_issuers_root,
            &leaf_hash,
            &merkle_path,
            &merkle_indices
        ),
        "Issuer is not in the trusted registry"
    );

    // 4. Verify Ed25519 Signature over the Payload
    let verifying_key =
        VerifyingKey::from_bytes(&issuer_pubkey_bytes).expect("Invalid issuer public key");
    let signature = Signature::from_bytes(&signature_bytes);

    verifying_key
        .verify(&payload_bytes, &signature)
        .expect("Invalid cryptographic signature");

    // 5. Hash the payload to generate the unique payload hash
    let mut hasher = Sha256::new();
    hasher.update(&payload_bytes);
    let payload_hash: [u8; 32] = hasher.finalize().into();

    // 6. Deserialize payload
    let payload: ThresholdPayload =
        bincode::deserialize(&payload_bytes).expect("Failed to deserialize payload");

    // 7. Compute the score
    let sum: i32 = payload.transactions.iter().sum();
    let score = sum / (payload.transactions.len() as i32).max(1);
    let threshold_met = score >= target_threshold;

    // 8. Commit ONLY the evaluation result (score is kept hidden)
    let output = ThresholdVerificationOutput {
        trusted_issuers_root,
        payload_hash,
        target_threshold,
        threshold_met,
        subject_id: payload.subject_id,
    };
    sp1_zkvm::io::commit(&output);
}
