#![allow(deprecated)]
#![cfg(test)]

use super::*;
use sadgi_registry::{ProgramRegistry, ProgramRegistryClient};
use sadgi_verifier::{Groth16Verifier, Groth16VerifierClient};
use soroban_sdk::{symbol_short, testutils::Address as _, Bytes, BytesN, Env, String};

#[test]
fn test_e2e_job_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    // 1. Setup Identities
    let developer = soroban_sdk::Address::generate(&env);
    let prover = soroban_sdk::Address::generate(&env);

    // 2. Deploy Infrastructure
    let registry_id = env.register_contract(None, ProgramRegistry);
    let registry_client = ProgramRegistryClient::new(&env, &registry_id);

    let verifier_id = env.register_contract(None, Groth16Verifier);
    let _verifier_client = Groth16VerifierClient::new(&env, &verifier_id);

    let marketplace_id = env.register_contract(None, SadgiMarketplace);
    let marketplace_client = SadgiMarketplaceClient::new(&env, &marketplace_id);

    // 3. Register a Program in the Registry
    let program_id = BytesN::from_array(&env, &[7; 32]);
    let vk = Bytes::from_slice(&env, &[1, 2, 3]); // Mock VK
    let metadata = String::from_str(&env, "E2E Test Program");
    registry_client.register(&program_id, &vk, &1, &metadata);

    // 4. Developer Creates Job
    let bounty = 500i128;
    let job_id = marketplace_client.create_job(&developer, &queue::JobClass::Standard, &bounty, &1);
    assert_eq!(job_id, 0); // Ledger sequence starts at 0

    // Force job state update (mocking the scheduler in a unit test context)
    // Actually, `assign_jobs` needs to be called.
    marketplace_client.assign_jobs(&job_id);

    // 5. Prover Submits Receipt
    // The verifier stub returns true if proof is not empty.
    let receipt = ProofReceipt {
        backend: sadgi_types::receipt::BackendType::SP1,
        program_id: program_id.clone(),
        program_version: 1,
        proof: Bytes::from_slice(&env, &[0xde, 0xad, 0xbe, 0xef]), // Non-empty proof
        public_values: Bytes::new(&env),
    };

    marketplace_client.submit_proof(&prover, &job_id, &receipt, &registry_id, &verifier_id);

    // 6. Verify Settlement
}
