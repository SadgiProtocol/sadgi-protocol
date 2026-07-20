# The Sadgi Job Standard (SIP-2)

A **Sadgi Job** is the on-chain representation of a compute request in the Soroban Marketplace. It defines what code needs to be executed, how much XLM is escrowed, and the operational parameters of the request.

## The Data Schema

```rust
use soroban_sdk::{contracttype, Address, BytesN};

#[contracttype]
pub struct JobRequest {
    /// A unique identifier for the Job (usually a hash of the parameters).
    pub id: BytesN<32>,
    
    /// The Soroban Address of the developer/contract funding the compute.
    pub sponsor: Address,
    
    /// The zkVM Image ID to execute.
    pub target_image_id: BytesN<32>,
    
    /// The XLM bounty locked in escrow.
    pub bounty: i128,
    
    /// The maximum ledger sequence number before the job expires.
    pub deadline_ledger: u32,
    
    /// Operational parameters (e.g., Single Prover vs. Fastest-Wins redundancy).
    pub mode: JobMode,
}

#[contracttype]
pub enum JobMode {
    /// A single Prover is selected. Highly efficient, lower cost.
    Standard,
    /// Multiple Provers are selected. The first to submit a valid receipt wins the bounty.
    FastestWins,
}
```

## The Escrow Lifecycle
1. **Queued**: The `sponsor` calls the Marketplace contract, locking the `bounty`.
2. **Assigned**: The Scheduler matches the `JobRequest` to active Provers.
3. **Fulfilled**: A Prover submits a valid `SadgiReceipt`. The `bounty` is unlocked and transferred.
4. **Expired**: If the `deadline_ledger` passes without a valid receipt, the `bounty` is refunded to the `sponsor`.
