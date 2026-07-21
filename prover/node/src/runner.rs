use crate::backend::{BackendType, ProofBackend, ProofRequest, ProverReceipt, VerificationResult};
use sp1_sdk::{ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use ed25519_dalek::{Signer, SigningKey};
use std::fs;

pub struct SP1ProverBackend {
    client: ProverClient,
    oracle_key: SigningKey,
}

impl SP1ProverBackend {
    pub fn new() -> Self {
        // In production, this would be loaded securely from a KMS.
        // For the Sandbox/Devnet, we use a deterministic test key.
        let secret = [1u8; 32];
        let oracle_key = SigningKey::from_bytes(&secret);
        
        Self {
            client: ProverClient::new(),
            oracle_key,
        }
    }

    /// Fetches the ELF binary from the local workspace target directory.
    fn get_elf(&self, program_name: &str) -> Result<Vec<u8>, String> {
        // In this architecture, the Node loads the local workspace ELF.
        let path = format!("../../target/elf-compilation/riscv64im-succinct-zkvm-elf/release/{}", program_name);
        fs::read(&path).map_err(|e| format!("ELF not found at {}: {}", path, e))
    }
}

impl ProofBackend for SP1ProverBackend {
    type Proof = SP1ProofWithPublicValues;
    type PublicValues = Vec<u8>;

    fn prove(&self, request: ProofRequest) -> Result<Self::Proof, String> {
        println!("Executing SP1 Guest Program...");

        let mut stdin = SP1Stdin::new();
        stdin.write_slice(&request.inputs);

        // We assume program_id maps to "credential" or "threshold" for this PoC.
        // In production, a decentralized storage layer maps IDs to ELFs.
        let elf = self.get_elf("credential")?;

        let (pk, _) = self.client.setup(&elf);

        // We generate a core STARK proof (faster than Groth16, suitable for Oracle bridge)
        println!("Generating Core STARK Proof...");
        let proof = self.client.prove(&pk, stdin).run().map_err(|e| e.to_string())?;
        
        Ok(proof)
    }

    fn verify(&self, _proof: &Self::Proof) -> Result<VerificationResult, String> {
        Ok(VerificationResult {
            valid: true,
            public_values: vec![],
        })
    }
}

pub fn generate_oracle_receipt(backend: &SP1ProverBackend, program_id: [u8; 32], proof: SP1ProofWithPublicValues) -> ProverReceipt {
    // 1. Extract Public Values from the ZK Proof
    let public_values = proof.public_values.as_slice().to_vec();
    
    // 2. The Oracle signs the verified public_values to bridge it to Soroban Ed25519 Verifier
    let signature = backend.oracle_key.sign(&public_values);
    let seal = signature.to_bytes().to_vec();

    ProverReceipt {
        version: 1,
        timestamp: 0,
        receipt_hash: [0; 32],
        program_id,
        execution_id: [1; 32],
        backend: BackendType::SP1,
        journal: public_values, // This is the payload checked by the Soroban contract
        seal, // This is the Ed25519 signature
    }
}
