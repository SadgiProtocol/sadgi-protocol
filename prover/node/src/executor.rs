use crate::backend::{BackendType, ProofBackend, ProofRequest, ProverReceipt};
use crate::runner;
use crate::scheduler::Job;

pub async fn execute_job(job: Job) -> ProverReceipt {
    match job.backend {
        BackendType::SP1 => {
            let backend = runner::SP1ProverBackend::new();
            let req = ProofRequest {
                program_id: job.program_id,
                program_version: job.program_version,
                inputs: vec![],
            };

            match backend.prove(req) {
                Ok(proof) => runner::generate_oracle_receipt(&backend, job.program_id, proof),
                Err(e) => {
                    println!("Proof generation failed: {}", e);
                    // In a real system we would return a failed receipt or error
                    panic!("Proof failed: {}", e);
                }
            }
        }
        _ => unimplemented!("Backend not yet supported"),
    }
}
