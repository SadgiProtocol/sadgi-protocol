use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CredentialPayload {
    pub subject_id: alloc::string::String,
    pub age: u8,
    pub status: alloc::string::String,
    pub expiration: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CredentialVerificationOutput {
    pub trusted_issuers_root: [u8; 32],
    pub credential_hash: [u8; 32],
    pub subject_id: alloc::string::String,
    pub age: u8,
    pub status: alloc::string::String,
    pub expiration: u64,
}
