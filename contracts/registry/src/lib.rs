#![allow(deprecated)]
#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Bytes, BytesN, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramRecord {
    pub verification_key: Bytes,
    pub version: u32,
    pub metadata: String,
}

#[contract]
pub struct ProgramRegistry;

#[contractimpl]
impl ProgramRegistry {
    /// Registers a new program or updates an existing one.
    /// In a production environment, this would require authentication (e.g. from the DAO or the original author).
    pub fn register(
        env: Env,
        program_id: BytesN<32>,
        verification_key: Bytes,
        version: u32,
        metadata: String,
    ) {
        let record = ProgramRecord {
            verification_key,
            version,
            metadata,
        };
        env.storage().persistent().set(&program_id, &record);
    }

    /// Retrieves the verification key and version for a given program.
    pub fn get_program(env: Env, program_id: BytesN<32>) -> Option<ProgramRecord> {
        env.storage().persistent().get(&program_id)
    }

    /// Convenience method for the marketplace to fetch just the VK.
    pub fn get_vk(env: Env, program_id: BytesN<32>) -> Bytes {
        let record: ProgramRecord = env.storage().persistent().get(&program_id).unwrap();
        record.verification_key
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_register_and_get() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ProgramRegistry);
        let client = ProgramRegistryClient::new(&env, &contract_id);

        let program_id = BytesN::from_array(&env, &[1; 32]);
        let vk = Bytes::from_slice(&env, &[2, 3, 4]);
        let metadata = String::from_str(&env, "test program");

        client.register(&program_id, &vk, &1, &metadata);

        let record = client.get_program(&program_id).unwrap();
        assert_eq!(record.version, 1);
        assert_eq!(record.verification_key, vk);
        assert_eq!(record.metadata, metadata);
    }
}
