#![no_std]

use sadgi_types::receipt::SadgiReceipt;
use soroban_sdk::{contractclient, Address, Env};

#[contractclient(name = "MarketplaceClient")]
pub trait MarketplaceTrait {
    fn create_job(env: Env, developer: Address, bounty: i128) -> u64;
    fn submit_proof(env: Env, prover: Address, job_id: u64, receipt: SadgiReceipt);
}

/// A simplified helper wrapper to make cross-contract calls from inside other Soroban contracts.
pub struct SadgiProtocol<'a> {
    env: &'a Env,
    contract_id: Address,
}

impl<'a> SadgiProtocol<'a> {
    pub fn new(env: &'a Env, contract_id: Address) -> Self {
        Self { env, contract_id }
    }

    /// Helper method to create a job and lock bounty.
    pub fn request_proof(&self, developer: &Address, bounty: i128) -> u64 {
        let client = MarketplaceClient::new(self.env, &self.contract_id);
        client.create_job(developer, &bounty)
    }

    /// Helper method for provers to submit a receipt.
    pub fn submit_receipt(&self, prover: &Address, job_id: u64, receipt: &SadgiReceipt) {
        let client = MarketplaceClient::new(self.env, &self.contract_id);
        client.submit_proof(prover, &job_id, receipt);
    }
}
