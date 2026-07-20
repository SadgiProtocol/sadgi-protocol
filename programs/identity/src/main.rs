#![no_main]

use risc0_zkvm::guest::env;
use sadgi_types::program::{CanonicalJournal, PrivateClaim, PublicRequirement, IssuerVerifier, MockVerifier};
use soroban_sdk::{BytesN, Symbol, Env};

risc0_zkvm::guest::entry!(main);

fn main() {
    let soroban_env = Env::default(); // Note: Env must be simulated inside the zkVM or abstracted

    // 1. Read Private Inputs (The hidden data)
    let private_claim: PrivateClaim = env::read();
    
    // 2. Read Public Inputs (The requirement requested by the dapp)
    let public_requirement: PublicRequirement = env::read();
    let issuer_pubkey: BytesN<32> = env::read();
    let subject_pubkey: BytesN<32> = env::read();
    let program_id: BytesN<32> = env::read();
    let timestamp: u64 = env::read(); // Current Unix Timestamp
    
    // 3. Verify Signature (Mocked for Demo, but uses the trait)
    if !MockVerifier::verify_claim(private_claim.value, &private_claim.issuer_signature, &issuer_pubkey) {
        panic!("InvalidSignature");
    }
    
    // 4. Evaluate Threshold (Age >= 18)
    // For Identity, `private_claim.value` represents the user's Age in years, or we can compute it from birth timestamp.
    // For simplicity of the generic abstraction, we assume `private_claim.value` is Age directly here.
    if private_claim.value < public_requirement.threshold {
        panic!("ThresholdNotMet");
    }
    
    // 5. Construct and Commit the Canonical Journal
    let journal = CanonicalJournal {
        program_id,
        subject_pubkey,
        claim_type: Symbol::new(&soroban_env, "AgeVerification"),
        result: true,
        timestamp,
        metadata_hash: BytesN::from_array(&soroban_env, &[0; 32]), // Abstracted metadata hash
    };
    
    env::commit(&journal);
}
