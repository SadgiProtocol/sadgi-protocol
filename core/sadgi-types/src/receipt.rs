use soroban_sdk::{contracttype, Bytes, BytesN, Env, Vec, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BackendType {
    RiscZero,
    SP1,
    Succinct,
    Custom,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceiptHeader {
    pub version: u32,
    pub timestamp: u64,
    pub receipt_hash: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceiptMetadata {
    pub program_id: BytesN<32>,
    pub execution_id: BytesN<32>,
    pub backend: BackendType,
}

/// A generic envelope for any Zero-Knowledge proof generated across different backends.
/// The marketplace and verifier smart contracts only ever deal with `SadgiReceipt`.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SadgiReceipt {
    pub header: ReceiptHeader,
    pub metadata: ReceiptMetadata,
    /// The public outputs committed by the guest program
    pub journal: Bytes,
    /// The opaque cryptographic proof bytes
    pub seal: Bytes,
}

impl SadgiReceipt {
    pub fn verify(&self, env: &Env) -> bool {
        // In a real implementation, this method would route the seal to the correct
        // verifier contract based on the `BackendType`. For now, this is an abstraction.
        true
    }
}
