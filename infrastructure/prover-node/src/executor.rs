use sadgi_types::receipt::SadgiReceipt;
use crate::scheduler::Job;
use crate::runner;

pub async fn execute_job(job: Job) -> SadgiReceipt {
    match job.backend {
        sadgi_types::receipt::BackendType::RiscZero => {
            runner::run_risc0_backend(job.program_id).await
        }
        _ => unimplemented!("Backend not yet supported"),
    }
}
