#![no_std]

use soroban_sdk::{contract, contractimpl, Bytes, BytesN, Env};

#[contract]
pub struct Groth16Verifier;

#[contractimpl]
impl Groth16Verifier {
    /// Verifies a ZK proof result.
    /// Since Soroban currently lacks BN254 pairing precompiles, this contract uses an Oracle-bridge pattern.
    /// The off-chain ZK prover generates the heavy proof, an oracle verifies it, and the oracle signs the `public_values`.
    /// `verification_key` is the Ed25519 public key of the Oracle (32 bytes).
    /// `proof` is the Ed25519 signature (64 bytes).
    /// `public_values` is the message that was signed.
    pub fn verify(env: Env, proof: Bytes, public_values: Bytes, verification_key: Bytes) -> bool {
        // 1. Validate lengths
        if verification_key.len() != 32 || proof.len() != 64 {
            return false;
        }

        // 2. Convert to fixed-size arrays
        let mut pk_bytes = [0u8; 32];
        verification_key.copy_into_slice(&mut pk_bytes);
        let pub_key = BytesN::from_array(&env, &pk_bytes);

        let mut sig_bytes = [0u8; 64];
        proof.copy_into_slice(&mut sig_bytes);
        let signature = BytesN::from_array(&env, &sig_bytes);

        // 3. Verify Ed25519 signature natively on Soroban
        env.crypto()
            .ed25519_verify(&pub_key, &public_values, &signature);

        true // If ed25519_verify fails, it panics and reverts the transaction (Soroban behavior)
    }
}
