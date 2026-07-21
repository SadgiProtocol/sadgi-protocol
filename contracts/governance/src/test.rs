#![allow(deprecated)]
#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env, symbol_short, vec, IntoVal};

// A mock protocol contract that governance will upgrade or call.
#[contract]
pub struct MockTargetContract;

#[contractimpl]
impl MockTargetContract {
    pub fn mock_upgrade(env: Env, new_wasm_hash: soroban_sdk::BytesN<32>) -> bool {
        // Return true if successful
        true
    }
}

#[test]
fn test_governance_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let proposer = Address::generate(&env);
    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);
    let executor = Address::generate(&env);

    let contract_id = env.register_contract(None, SadgiGovernance);
    let client = SadgiGovernanceClient::new(&env, &contract_id);

    client.init(&admin);

    let target_id = env.register_contract(None, MockTargetContract);
    
    // Propose an upgrade
    let mock_hash = soroban_sdk::BytesN::from_array(&env, &[0; 32]);
    let args: Vec<Val> = vec![&env, mock_hash.into_val(&env)];
    
    let proposal_id = client.propose(
        &proposer,
        &target_id,
        &symbol_short!("mock_upgrade"),
        &args,
    );
    assert_eq!(proposal_id, 1);

    // Vote
    client.vote(&voter1, &proposal_id, &true);
    client.vote(&voter2, &proposal_id, &true);

    let p = client.get_proposal(&proposal_id);
    assert_eq!(p.yes_votes, 2);
    assert_eq!(p.executed, false);

    // Execute
    client.execute(&executor, &proposal_id);

    let p_after = client.get_proposal(&proposal_id);
    assert_eq!(p_after.executed, true);
}
