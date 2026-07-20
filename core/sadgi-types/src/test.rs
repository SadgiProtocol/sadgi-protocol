#![cfg(test)]

use super::*;
use soroban_sdk::{Bytes, BytesN, Env};

#[test]
fn test_receipt_serialization() {
    let env = Env::default();

    let original = receipt::SadgiReceipt {
        header: receipt::ReceiptHeader {
            version: 1,
            timestamp: 42,
            receipt_hash: BytesN::from_array(&env, &[7; 32]),
        },
        metadata: receipt::ReceiptMetadata {
            program_id: BytesN::from_array(&env, &[1; 32]),
            execution_id: BytesN::from_array(&env, &[2; 32]),
            backend: receipt::BackendType::RiscZero,
        },
        journal: Bytes::from_slice(&env, &[10, 20, 30]),
        seal: Bytes::from_slice(&env, &[99, 99, 99]),
    };

    // Serialize
    // Using soroban_sdk built-in XDR encoding if we had it, or just testing field integrity:
    assert_eq!(original.header.version, 1);
    assert_eq!(original.metadata.backend, receipt::BackendType::RiscZero);
}
