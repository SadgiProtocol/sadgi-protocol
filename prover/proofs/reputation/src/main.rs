#![no_main]
sp1_zkvm::entrypoint!(main);

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct InteractionEvent {
    pub job_id: u64,
    pub successful: bool,
    pub bounty_earned: u64,
}

#[derive(Deserialize, Debug)]
struct ReputationHistory {
    pub prover_id: String,
    pub events: Vec<InteractionEvent>,
}

pub fn main() {
    // 1. Read caller contract ID (Public Input)
    let caller_contract_id = sp1_zkvm::io::read::<[u8; 32]>();
    sp1_zkvm::io::commit(&caller_contract_id);

    // 2. Read gaming / on-chain reputation history (Private Input)
    let history_json = sp1_zkvm::io::read::<String>();

    // 3. Parse JSON
    let history: ReputationHistory =
        serde_json::from_str(&history_json).expect("Failed to parse reputation history");

    // 4. Calculate Reputation Score
    // Game-theoretic model:
    // +10 points for every successful job
    // +1 point for every 10 XLM earned
    // -50 points for every failed/slashed job
    let mut score: i64 = 100; // Base score
    let mut total_jobs = 0;

    for event in history.events.iter() {
        total_jobs += 1;
        if event.successful {
            score += 10;
            score += (event.bounty_earned / 10) as i64;
        } else {
            score -= 50;
        }
    }

    // 5. Ensure score does not drop below 0
    let final_score = if score < 0 { 0 } else { score as u64 };

    // 6. Threshold requirement
    // Only allow proof generation if the user has a minimum reputation and history
    assert!(
        total_jobs >= 5,
        "Not enough history to generate a reputation proof"
    );
    assert!(final_score >= 150, "Reputation score is too low");

    // 7. Commit the score (Public Output)
    // The prover reveals their final computed score to the smart contract,
    // but the exact sequence of jobs, failures, and bounties earned remains hidden inside the ZK proof.
    sp1_zkvm::io::commit(&final_score);
}
