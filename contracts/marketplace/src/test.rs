#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Events, Env};

#[test]
fn test_job_creation_and_fulfillment() {
    let env = Env::default();
    env.mock_all_auths();

    let developer = soroban_sdk::Address::generate(&env);
    let prover = soroban_sdk::Address::generate(&env);

    let contract_id = env.register_contract(None, SadgiMarketplace);
    let client = SadgiMarketplaceClient::new(&env, &contract_id);

    // 1. Developer Creates Job
    let job_id = client.create_job(&developer, &100i128);
    assert_eq!(job_id, 1);

    let events = env.events().all();
    assert!(events.len() > 0);

    // 2. Mock a generic Receipt (backend agnostic)
    let receipt = SadgiReceipt {
        header: sadgi_types::receipt::ReceiptHeader {
            version: 1,
            timestamp: 123456789,
            receipt_hash: soroban_sdk::BytesN::from_array(&env, &[1; 32]),
        },
        metadata: sadgi_types::receipt::ReceiptMetadata {
            program_id: soroban_sdk::BytesN::from_array(&env, &[0; 32]),
            execution_id: soroban_sdk::BytesN::from_array(&env, &[2; 32]),
            backend: sadgi_types::receipt::BackendType::RiscZero,
        },
        journal: soroban_sdk::Bytes::new(&env),
        seal: soroban_sdk::Bytes::new(&env),
    };

    // 3. Prover Submits Receipt
    client.submit_proof(&prover, &job_id, &receipt);

    // If verified, events should include ProofVerified
    let final_events = env.events().all();
    assert!(final_events.len() > 1);
}
