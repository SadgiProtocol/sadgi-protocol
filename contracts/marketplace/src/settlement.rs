use soroban_sdk::{Address, Env};
use crate::scheduler::{Scheduler, ProverProfile};

pub struct Settlement;

impl Settlement {
    /// Disburse funds for a successfully verified proof.
    /// Uses dynamic pricing: % to Prover, % to Treasury (Governance).
    pub fn release_funds(env: &Env, prover: Address, bounty: i128, fee_percent: u32) {
        let treasury_fee = (bounty * (fee_percent as i128)) / 100;
        let prover_reward = bounty - treasury_fee;
        
        // In a real contract, we would use token::Client to transfer XLM.
        // token::Client::new(env, &xlm_address).transfer(&env.current_contract_address(), &prover, &prover_reward);
        // token::Client::new(env, &xlm_address).transfer(&env.current_contract_address(), &treasury_address, &treasury_fee);
        
        // Increment Reputation for success
        if let Some(mut profile) = Scheduler::get_prover(env, &prover) {
            profile.total_successful_jobs += 1;
            profile.active_jobs -= 1;
            if profile.reputation_score < 100 {
                profile.reputation_score += 1;
            }
            Scheduler::update_prover(env, &prover, profile);
        }
    }
    
    /// Progressively penalize a prover for failing to submit or submitting invalid proofs.
    /// Malicious (fraud) vs Timeout dictates the severity.
    pub fn penalize_prover(env: &Env, prover: Address, is_malicious: bool) {
        if let Some(mut profile) = Scheduler::get_prover(env, &prover) {
            profile.total_failed_jobs += 1;
            profile.active_jobs -= 1;
            
            if is_malicious {
                // Fraud: Maximum penalty, 50% slash, instant suspension
                profile.is_suspended = true;
                profile.reputation_score = 0;
                let slash_amount = profile.staked_xlm / 2;
                profile.staked_xlm -= slash_amount;
                
                // Route slash_amount to Treasury / Developer Compensation
            } else {
                // Timeout/Hardware Crash: Warning and reputation hit
                if profile.reputation_score > 10 {
                    profile.reputation_score -= 10;
                } else {
                    profile.reputation_score = 0;
                }
                
                // If reputation falls too low, suspend them
                if profile.reputation_score < 20 {
                    profile.is_suspended = true;
                }
            }
            
            Scheduler::update_prover(env, &prover, profile);
        }
    }
}
