use sadgi_types::receipt::SadgiReceipt;

#[derive(Debug)]
pub struct Job {
    pub id: u64,
    pub program_id: [u8; 32],
    pub backend: sadgi_types::receipt::BackendType,
}

pub struct Scheduler {
    // In a real implementation, this holds Stellar RPC clients to poll the contract
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {}
    }

    pub async fn poll_next_job(&mut self) -> Option<Job> {
        // Mocking finding a job on the marketplace
        None
    }

    pub async fn submit_receipt(&self, receipt: SadgiReceipt) {
        println!("Submitting Receipt to Soroban Marketplace: {:?}", receipt.header.receipt_hash);
    }
}
