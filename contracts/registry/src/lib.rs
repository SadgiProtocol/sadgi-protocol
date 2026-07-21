#![allow(deprecated)]
#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, Bytes, BytesN, Env, String,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    ProgramNotFound = 1,
    Unauthorized = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramRecord {
    pub author: Address,
    pub verification_key: Bytes,
    pub version: u32,
    pub metadata: String,
}

#[contract]
pub struct ProgramRegistry;

#[contractimpl]
impl ProgramRegistry {
    /// Registers a new program or updates an existing one.
    /// Requires authentication from the original author to update.
    pub fn register(
        env: Env,
        author: Address,
        program_id: BytesN<32>,
        verification_key: Bytes,
        version: u32,
        metadata: String,
    ) -> Result<(), Error> {
        author.require_auth();

        if let Some(existing) = env
            .storage()
            .persistent()
            .get::<_, ProgramRecord>(&program_id)
        {
            if existing.author != author {
                return Err(Error::Unauthorized);
            }
        }

        let record = ProgramRecord {
            author,
            verification_key,
            version,
            metadata,
        };
        env.storage().persistent().set(&program_id, &record);
        Ok(())
    }

    /// Retrieves the verification key and version for a given program.
    pub fn get_program(env: Env, program_id: BytesN<32>) -> Option<ProgramRecord> {
        env.storage().persistent().get(&program_id)
    }

    /// Convenience method for the marketplace to fetch just the VK.
    pub fn get_vk(env: Env, program_id: BytesN<32>) -> Result<Bytes, Error> {
        let record: ProgramRecord = env
            .storage()
            .persistent()
            .get(&program_id)
            .ok_or(Error::ProgramNotFound)?;
        Ok(record.verification_key)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_register_and_get() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, ProgramRegistry);
        let client = ProgramRegistryClient::new(&env, &contract_id);

        let author = soroban_sdk::testutils::Address::generate(&env);
        let program_id = BytesN::from_array(&env, &[1; 32]);
        let vk = Bytes::from_slice(&env, &[2, 3, 4]);
        let metadata = String::from_str(&env, "test program");

        client.register(&author, &program_id, &vk, &1, &metadata);

        let record = client.get_program(&program_id).unwrap();
        assert_eq!(record.author, author);
        assert_eq!(record.version, 1);
        assert_eq!(record.verification_key, vk);
        assert_eq!(record.metadata, metadata);
    }
}
