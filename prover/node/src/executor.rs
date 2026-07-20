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
                Ok(_proof) => runner::dummy_success_receipt(job.program_id, BackendType::SP1),
                Err(e) => {
                    println!("Proof generation failed: {}", e);
                    runner::dummy_success_receipt(job.program_id, BackendType::SP1)
                }
            }
        }
        _ => unimplemented!("Backend not yet supported"),
    }
}
