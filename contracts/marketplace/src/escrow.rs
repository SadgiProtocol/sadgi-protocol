use soroban_sdk::{symbol_short, token, Address, Env};

pub struct Escrow;

impl Escrow {
    pub fn lock_funds(env: &Env, developer: Address, amount: i128) {
        developer.require_auth();

        let key_token = symbol_short!("token_addr");
        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&key_token)
            .expect("Marketplace not initialized");

        // Transfer funds from Developer to Escrow (this contract)
        let token = token::Client::new(env, &token_addr);
        token.transfer(&developer, &env.current_contract_address(), &amount);

        env.storage().instance().set(&developer, &amount);
    }
}
