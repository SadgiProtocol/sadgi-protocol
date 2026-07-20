use soroban_sdk::contracttype;

/// The canonical state machine for a Verifiable Compute Job in the Sadgi Protocol.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JobState {
    /// Developer requested a proof, XLM is locked in Escrow.
    Request,
    /// Job is sitting in the mempool waiting for a Prover.
    Queue,
    /// A specific Prover has claimed the job and staked XLM.
    Assigned,
    /// The Prover is currently generating the proof off-chain.
    Executing,
    /// The Prover generated the receipt and submitted it to the network.
    Submitted,
    /// The `SadgiReceipt` has been mathematically verified on-chain.
    Verified,
    /// Escrow funds have been released to the Prover, and stake returned.
    Settled,
    /// The job is finalized and stored in the registry history.
    Archived,
    /// The job failed (e.g., timeout, invalid proof). Prover stake is slashed.
    Failed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentState {
    Unfunded,
    Locked,
    Released,
    Refunded,
    Slashed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProverState {
    Idle,
    Working,
    Slashed,
    Banned,
}
