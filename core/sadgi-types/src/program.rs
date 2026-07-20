#![no_std]

use soroban_sdk::{contracttype, Bytes, BytesN, Symbol};

/// The Canonical Journal structure that EVERY reference program must commit.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalJournal {
    pub program_id: BytesN<32>,
    pub subject_pubkey: BytesN<32>,
    pub claim_type: Symbol,
    pub result: bool,
    pub timestamp: u64,
    pub metadata_hash: BytesN<32>,
}

/// Generic structure for a Private Claim provided by the user.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrivateClaim {
    /// The actual hidden value (e.g., age or credit score).
    pub value: u64,
    /// Cryptographic signature from the Trusted Issuer asserting this value is true.
    pub issuer_signature: Bytes,
}

/// Generic structure for the Public Requirement the user is attempting to prove.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicRequirement {
    /// The threshold the user must meet (e.g., 18 for age, 700 for credit).
    pub threshold: u64,
    /// Operator: e.g., 0 for Equal, 1 for GreaterThanOrEqual
    pub operator: u32,
}

/// Pluggable interface for verifying Issuer Signatures within the zkVM.
pub trait IssuerVerifier {
    /// Verifies that the `signature` correctly signed the `value` by the trusted `issuer_pubkey`.
    fn verify_claim(value: u64, signature: &Bytes, issuer_pubkey: &BytesN<32>) -> bool;
}

/// A Mock verifier used strictly for local development and reference testing.
pub struct MockVerifier;

impl IssuerVerifier for MockVerifier {
    fn verify_claim(_value: u64, _signature: &Bytes, _issuer_pubkey: &BytesN<32>) -> bool {
        // MOCK: Always succeeds. In production, use Secp256k1Verifier or Ed25519Verifier.
        true
    }
}
