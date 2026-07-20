#![no_std]

pub mod client;

use sadgi_types::receipt::{BackendType, ProofReceipt};
use soroban_sdk::{Bytes, BytesN, Env};

/// Builder pattern for safely constructing a generic `ProofReceipt`.
pub struct ReceiptBuilder<'a> {
    env: &'a Env,
    backend: BackendType,
    program_id: Option<BytesN<32>>,
    program_version: u32,
    proof: Option<Bytes>,
    public_values: Option<Bytes>,
}

impl<'a> ReceiptBuilder<'a> {
    pub fn new(env: &'a Env) -> Self {
        Self {
            env,
            backend: BackendType::SP1, // Default backend
            program_id: None,
            program_version: 1,
            proof: None,
            public_values: None,
        }
    }

    pub fn with_backend(mut self, backend: BackendType) -> Self {
        self.backend = backend;
        self
    }

    pub fn with_program_id(mut self, id: [u8; 32]) -> Self {
        self.program_id = Some(BytesN::from_array(self.env, &id));
        self
    }

    pub fn with_program_version(mut self, version: u32) -> Self {
        self.program_version = version;
        self
    }

    pub fn with_proof(mut self, proof: &[u8]) -> Self {
        self.proof = Some(Bytes::from_slice(self.env, proof));
        self
    }

    pub fn with_public_values(mut self, public_values: &[u8]) -> Self {
        self.public_values = Some(Bytes::from_slice(self.env, public_values));
        self
    }

    pub fn build(self) -> ProofReceipt {
        ProofReceipt {
            backend: self.backend,
            program_id: self
                .program_id
                .unwrap_or_else(|| BytesN::from_array(self.env, &[0; 32])),
            program_version: self.program_version,
            proof: self.proof.unwrap_or_else(|| Bytes::new(self.env)),
            public_values: self.public_values.unwrap_or_else(|| Bytes::new(self.env)),
        }
    }
}
