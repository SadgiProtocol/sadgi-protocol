#![no_std]

pub mod client;

use sadgi_types::receipt::{BackendType, ReceiptHeader, ReceiptMetadata, SadgiReceipt};
use soroban_sdk::{Bytes, BytesN, Env};

/// Builder pattern for safely constructing a generic `SadgiReceipt`.
pub struct ReceiptBuilder<'a> {
    env: &'a Env,
    version: u32,
    timestamp: u64,
    receipt_hash: Option<BytesN<32>>,
    program_id: Option<BytesN<32>>,
    execution_id: Option<BytesN<32>>,
    backend: BackendType,
    journal: Option<Bytes>,
    seal: Option<Bytes>,
}

impl<'a> ReceiptBuilder<'a> {
    pub fn new(env: &'a Env) -> Self {
        Self {
            env,
            version: 1,
            timestamp: 0,
            receipt_hash: None,
            program_id: None,
            execution_id: None,
            backend: BackendType::RiscZero, // Default backend
            journal: None,
            seal: None,
        }
    }

    pub fn with_version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn with_hash(mut self, hash: [u8; 32]) -> Self {
        self.receipt_hash = Some(BytesN::from_array(self.env, &hash));
        self
    }

    pub fn with_program_id(mut self, id: [u8; 32]) -> Self {
        self.program_id = Some(BytesN::from_array(self.env, &id));
        self
    }

    pub fn with_execution_id(mut self, id: [u8; 32]) -> Self {
        self.execution_id = Some(BytesN::from_array(self.env, &id));
        self
    }

    pub fn with_backend(mut self, backend: BackendType) -> Self {
        self.backend = backend;
        self
    }

    pub fn with_journal(mut self, journal: &[u8]) -> Self {
        self.journal = Some(Bytes::from_slice(self.env, journal));
        self
    }

    pub fn with_seal(mut self, seal: &[u8]) -> Self {
        self.seal = Some(Bytes::from_slice(self.env, seal));
        self
    }

    pub fn build(self) -> SadgiReceipt {
        SadgiReceipt {
            header: ReceiptHeader {
                version: self.version,
                timestamp: self.timestamp,
                receipt_hash: self
                    .receipt_hash
                    .unwrap_or_else(|| BytesN::from_array(self.env, &[0; 32])),
            },
            metadata: ReceiptMetadata {
                program_id: self
                    .program_id
                    .unwrap_or_else(|| BytesN::from_array(self.env, &[0; 32])),
                execution_id: self
                    .execution_id
                    .unwrap_or_else(|| BytesN::from_array(self.env, &[0; 32])),
                backend: self.backend,
            },
            journal: self.journal.unwrap_or_else(|| Bytes::new(self.env)),
            seal: self.seal.unwrap_or_else(|| Bytes::new(self.env)),
        }
    }
}
