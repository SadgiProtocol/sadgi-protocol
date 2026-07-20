use soroban_sdk::{contracttype, Address, BytesN};

/// Standard Marketplace Events emitted during the lifecycle of a job.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProtocolEvent {
    /// Developer posted a job and locked funds.
    JobCreated {
        job_id: u64,
        developer: Address,
        program_id: BytesN<32>,
        bounty: i128,
    },
    /// A Prover was selected from the queue.
    JobAssigned { job_id: u64, prover: Address },
    /// Prover generated the receipt and submitted it.
    ProofSubmitted {
        job_id: u64,
        receipt_hash: BytesN<32>,
    },
    /// Verifier mathematically verified the receipt.
    ProofVerified { job_id: u64 },
    /// Funds have been sent to the prover.
    RewardPaid {
        job_id: u64,
        prover: Address,
        amount: i128,
    },
    /// Job timed out or prover submitted invalid proof.
    JobFailed {
        job_id: u64,
        reason: soroban_sdk::Symbol,
    },
}
