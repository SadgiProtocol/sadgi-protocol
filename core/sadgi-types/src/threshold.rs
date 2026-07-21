use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThresholdPayload {
    pub subject_id: alloc::string::String,
    pub transactions: alloc::vec::Vec<i32>,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThresholdVerificationOutput {
    pub trusted_issuers_root: [u8; 32],
    pub payload_hash: [u8; 32],
    pub target_threshold: i32,
    pub threshold_met: bool,
    pub subject_id: alloc::string::String,
}
