use soroban_sdk::{contracttype, Address, BytesN};

/// Standard Marketplace Events emitted during the lifecycle of a job.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProtocolEvent {
    /// Developer posted a job and locked funds.
    JobCreated(u64, Address, BytesN<32>, i128),
    /// A Prover was selected from the queue.
    JobAssigned(u64, Address),
    /// Prover generated the receipt and submitted it.
    ProofSubmitted(u64, BytesN<32>),
    /// Verifier mathematically verified the receipt.
    ProofVerified(u64),
    /// Funds have been sent to the prover.
    RewardPaid(u64, Address, i128),
    /// Job timed out or prover submitted invalid proof.
    JobFailed(u64, soroban_sdk::Symbol),
}
