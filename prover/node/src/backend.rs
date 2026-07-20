use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum BackendType {
    SP1,
    Succinct,
    Custom,
}

#[derive(Debug, Clone)]
pub struct ProverReceipt {
    pub version: u32,
    pub timestamp: u64,
    pub receipt_hash: [u8; 32],
    pub program_id: [u8; 32],
    pub execution_id: [u8; 32],
    pub backend: BackendType,
    pub journal: Vec<u8>,
    pub seal: Vec<u8>,
}

pub struct ProofRequest {
    pub program_id: [u8; 32],
    pub program_version: u32,
    pub inputs: Vec<u8>,
}

pub struct VerificationResult {
    pub valid: bool,
    pub public_values: Vec<u8>,
}

pub trait ProofBackend {
    type Proof: Debug;
    type PublicValues: Debug;

    fn prove(&self, request: ProofRequest) -> Result<Self::Proof, String>;
    fn verify(&self, proof: &Self::Proof) -> Result<VerificationResult, String>;
}
