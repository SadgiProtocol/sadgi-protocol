use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum BackendType {
    SP1,
    Succinct,
    Custom,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
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

#[allow(dead_code)]
pub struct ProofRequest {
    pub program_id: [u8; 32],
    pub program_version: u32,
    pub inputs: Vec<u8>,
}

#[allow(dead_code)]
pub struct VerificationResult {
    pub valid: bool,
    pub public_values: Vec<u8>,
}

pub trait ProofBackend {
    type Proof: Debug;
    type PublicValues: Debug;

    async fn prove(&self, request: ProofRequest) -> Result<Self::Proof, String>;
    #[allow(dead_code)]
    fn verify(&self, proof: &Self::Proof) -> Result<VerificationResult, String>;
}
