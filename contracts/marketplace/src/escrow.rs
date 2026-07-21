use soroban_sdk::{Address, Env};

pub struct Escrow;

impl Escrow {
    pub fn lock_funds(env: &Env, developer: Address, amount: i128) {
        developer.require_auth();
        // In reality, this transfers Stellar USDC or XLM to the contract address
        env.storage().instance().set(&developer, &amount);
    }
}
