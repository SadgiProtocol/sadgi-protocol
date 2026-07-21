use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;
use sadgi_types::credential::{CredentialPayload, CredentialVerificationOutput};
use sha2::{Digest, Sha256};
use sp1_sdk::{Prover, ProverClient, SP1Stdin};

use std::fs;

#[tokio::test]
async fn test_credential_verification_success() {
    // 1. Setup Keys
    let mut csprng = OsRng;
    let issuer_keypair = SigningKey::generate(&mut csprng);
    let issuer_pubkey = issuer_keypair.verifying_key();
    let issuer_pubkey_bytes = issuer_pubkey.to_bytes();

    // 2. Setup Merkle Tree (Depth 2 for test)
    let mut hasher = Sha256::new();
    hasher.update(&issuer_pubkey_bytes);
    let leaf0: [u8; 32] = hasher.finalize().into(); // Our trusted issuer

    let leaf1 = [1u8; 32]; // Dummy issuer
    let leaf2 = [2u8; 32]; // Dummy issuer
    let leaf3 = [3u8; 32]; // Dummy issuer

    // Hash pairs
    let mut hasher = Sha256::new();
    hasher.update(leaf0);
    hasher.update(leaf1);
    let node01: [u8; 32] = hasher.finalize().into();

    let mut hasher = Sha256::new();
    hasher.update(leaf2);
    hasher.update(leaf3);
    let node23: [u8; 32] = hasher.finalize().into();

    // Root
    let mut hasher = Sha256::new();
    hasher.update(node01);
    hasher.update(node23);
    let trusted_issuers_root: [u8; 32] = hasher.finalize().into();

    let merkle_path = vec![leaf1, node23];
    let merkle_indices = vec![true, true]; // leaf1 is on the right of leaf0, node23 is on the right of node01

    // 3. Create Payload
    let payload = CredentialPayload {
        subject_id: "did:sadgi:user-123".to_string(),
        age: 25,
        status: "verified".to_string(),
        expiration: 5000,
    };
    let payload_bytes = bincode::serialize(&payload).unwrap();

    // 4. Sign Payload
    let signature = issuer_keypair.sign(&payload_bytes);
    let signature_bytes = signature.to_bytes();

    // 5. Setup SP1 ZKVM Inputs
    let mut stdin = SP1Stdin::new();

    // Public Inputs (Policies)
    stdin.write(&trusted_issuers_root);

    // Private Inputs (Witnesses)
    stdin.write(&payload_bytes);
    stdin.write(&signature_bytes.to_vec());
    stdin.write(&issuer_pubkey_bytes);
    stdin.write(&merkle_path);
    stdin.write(&merkle_indices);

    // 6. Execute Program
    let elf_path = std::path::PathBuf::from(
        "../../prover/target/elf-compilation/riscv64im-succinct-zkvm-elf/release/credential",
    );
    let elf = fs::read(&elf_path)
        .expect("Failed to read ELF. Run `cargo prove build` in prover/proofs/credential first.");
    let elf_bytes: &'static [u8] = Box::leak(elf.into_boxed_slice());

    let client = sp1_sdk::ProverClient::from_env().await;
    let (mut public_values, execution_report) = client
        .execute(sp1_sdk::Elf::Static(elf_bytes), stdin)
        .await
        .unwrap();

    println!(
        "Execution completed successfully! Cycles: {}",
        execution_report.total_instruction_count()
    );
    let output = public_values.read::<CredentialVerificationOutput>();
    println!("Public Output: {:#?}", output);

    // 7. Verify Public Output
    assert_eq!(output.trusted_issuers_root, trusted_issuers_root);
    assert_eq!(output.subject_id, "did:sadgi:user-123");
    assert_eq!(output.age, 25);
    assert_eq!(output.status, "verified");
    assert_eq!(output.expiration, 5000);

    // Ensure credential hash is correct
    let mut hasher = Sha256::new();
    hasher.update(&payload_bytes);
    let expected_credential_hash: [u8; 32] = hasher.finalize().into();
    assert_eq!(output.credential_hash, expected_credential_hash);
}
