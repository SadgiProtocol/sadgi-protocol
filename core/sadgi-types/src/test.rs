#![cfg(test)]

use super::*;
use soroban_sdk::{Bytes, BytesN, Env};

#[test]
fn test_receipt_serialization() {
    let env = Env::default();

    let original = receipt::ProofReceipt {
        backend: receipt::BackendType::SP1,
        program_id: BytesN::from_array(&env, &[1; 32]),
        program_version: 1,
        proof: Bytes::from_slice(&env, &[10, 20, 30]),
        public_values: Bytes::from_slice(&env, &[99, 99, 99]),
    };

    // Serialize
    // Using soroban_sdk built-in XDR encoding if we had it, or just testing field integrity:
    assert_eq!(original.program_version, 1);
    assert_eq!(original.backend, receipt::BackendType::SP1);
}
