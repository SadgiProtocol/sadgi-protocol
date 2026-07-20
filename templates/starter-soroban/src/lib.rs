#![no_std]

use sadgi_types::receipt::SadgiReceipt;
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct StarterContract;

#[contractimpl]
impl StarterContract {
    /// Step 1: An end-user calls your smart contract to trigger an action 
    /// that requires a zero-knowledge proof.
    pub fn trigger_action(env: Env, user: Address) {
        user.require_auth();
        
        // In a production app, you would make a Cross-Contract Call here 
        // to the Sadgi Marketplace to queue a JobRequest and escrow XLM.
        // e.g., sadgi_marketplace_client.create_job(...)
    }

    /// Step 2: Once the Sadgi Prover finishes computing the job off-chain, 
    /// they submit the `SadgiReceipt` to the Marketplace. The Marketplace 
    /// verifies it and can trigger a callback to this function.
    pub fn process_receipt(env: Env, receipt: SadgiReceipt) {
        // 1. Verify the Receipt using the official Sadgi Verifier Contract.
        // let is_valid = sadgi_verifier_client.verify(&receipt);
        // assert!(is_valid, "Invalid Zero-Knowledge Proof");
        
        // 2. Prevent Replay Attacks (SIP-1 Compliance).
        // Ensure the receipt was explicitly generated for THIS contract.
        let expected_caller_id = env.current_contract_address().to_xdr(&env);
        // assert_eq!(receipt.journal.slice(0..32), expected_caller_id);
        
        // 3. Process the business logic!
        // The proof is mathematically guaranteed to be correct.
        // You can safely read the `receipt.journal` and update your contract state.
    }
}
