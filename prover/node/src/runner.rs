use crate::backend::{BackendType, ProofBackend, ProofRequest, ProverReceipt, VerificationResult};
use sp1_sdk::{ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use std::fs;
use std::process::Command;

pub struct SP1ProverBackend {
    client: ProverClient,
}

impl SP1ProverBackend {
    pub fn new() -> Self {
        Self {
            client: ProverClient::new(),
        }
    }

    /// Helper to check if Docker is running, which is required for Groth16 proofs via Succinct's containers.
    fn is_docker_running(&self) -> bool {
        Command::new("docker")
            .arg("info")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Fetches the ELF binary from the local registry based on ID and Version.
    fn get_elf(&self, program_id: [u8; 32], version: u32) -> Result<Vec<u8>, String> {
        let hex_id = hex::encode(program_id);
        let path = format!("./elfs/{}/v{}.elf", hex_id, version);

        // Mock returning an empty ELF for the stub, or read it if it exists.
        fs::read(&path).or_else(|_| {
            println!("Warning: ELF not found at {}. Using mock ELF.", path);
            Ok(vec![])
        })
    }
}

impl ProofBackend for SP1ProverBackend {
    type Proof = SP1ProofWithPublicValues;
    type PublicValues = Vec<u8>;

    fn prove(&self, request: ProofRequest) -> Result<Self::Proof, String> {
        println!("Executing SP1 Guest Program...");

        let mut stdin = SP1Stdin::new();
        stdin.write_slice(&request.inputs);

        let elf = self.get_elf(request.program_id, request.program_version)?;

        // This fails if elf is empty, but we'll leave it as a stub.
        /*
        let (pk, _) = self.client.setup(&elf);

        let proof = if self.is_docker_running() {
            println!("Docker detected. Generating Groth16 Proof...");
            self.client.prove(&pk, stdin).groth16().run().map_err(|e| e.to_string())?
        } else {
            println!("Docker not detected. Generating Core Proof...");
            self.client.prove(&pk, stdin).run().map_err(|e| e.to_string())?
        };
        Ok(proof)
        */

        Err("SP1 Prove Stub: Provide real ELF".to_string())
    }

    fn verify(&self, _proof: &Self::Proof) -> Result<VerificationResult, String> {
        Ok(VerificationResult {
            valid: true,
            public_values: vec![],
        })
    }
}

pub fn dummy_success_receipt(program_id: [u8; 32], backend_type: BackendType) -> ProverReceipt {
    ProverReceipt {
        version: 1,
        timestamp: 0,
        receipt_hash: [0; 32],
        program_id,
        execution_id: [1; 32],
        backend: backend_type,
        journal: vec![],
        seal: vec![],
    }
}
