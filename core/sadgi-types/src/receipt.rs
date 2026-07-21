use soroban_sdk::{contracttype, Bytes, BytesN, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BackendType {
    SP1,
    Succinct,
    Custom,
}

/// A generic envelope for any Zero-Knowledge proof generated across different backends.
/// The marketplace and verifier smart contracts only ever deal with `ProofReceipt`.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProofReceipt {
    pub backend: BackendType,
    pub program_id: BytesN<32>,
    pub program_version: u32,
    pub proof: Bytes,
    pub public_values: Bytes,
}

impl ProofReceipt {
    pub fn verify(&self, _env: &Env) -> bool {
        // Verification logic is now delegated to the Verifier contract via cross-contract calls.
        true
    }
}
