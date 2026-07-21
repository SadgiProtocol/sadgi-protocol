#![allow(clippy::manual_find)]
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProverProfile {
    pub prover_address: Address,
    pub staked_xlm: i128,
    pub active_jobs: u32,

    // Capacity Profile
    pub max_concurrency: u32,
    pub cpu_cores: u32,
    pub memory_mb: u32,

    // Reputation & Risk Tracking
    pub total_successful_jobs: u32,
    pub total_failed_jobs: u32,
    pub reputation_score: u32, // 0 to 100
    pub is_suspended: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Prover(Address),
    ProverList,
}

pub struct Scheduler;

impl Scheduler {
    /// Calculate the dynamic required stake for a prover based on load and reputation
    pub fn get_required_stake(base_stake: i128, concurrent_jobs: u32, reputation: u32) -> i128 {
        // High reputation (e.g. 100) reduces stake requirement, low reputation increases it.
        // For example: multiplier = 2.0 - (reputation / 100)
        let rep_multiplier = 200 - reputation as i128; // ranges from 100 to 200
        (base_stake * (concurrent_jobs as i128) * rep_multiplier) / 100
    }

    pub fn register_prover(env: &Env, profile: ProverProfile) {
        // Store the profile using O(1) indexed key
        env.storage()
            .persistent()
            .set(&DataKey::Prover(profile.prover_address.clone()), &profile);

        // Add address to the iterable list if not present
        let mut prover_list: soroban_sdk::Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::ProverList)
            .unwrap_or_else(|| soroban_sdk::Vec::new(env));

        if !prover_list.contains(&profile.prover_address) {
            prover_list.push_back(profile.prover_address.clone());
            env.storage()
                .persistent()
                .set(&DataKey::ProverList, &prover_list);
        }
    }

    pub fn get_prover(env: &Env, prover_address: &Address) -> Option<ProverProfile> {
        env.storage()
            .persistent()
            .get(&DataKey::Prover(prover_address.clone()))
    }

    pub fn update_prover(env: &Env, prover_address: &Address, updated_profile: ProverProfile) {
        // O(1) update
        env.storage()
            .persistent()
            .set(&DataKey::Prover(prover_address.clone()), &updated_profile);
    }

    /// Select the best Provers based on capacity, reputation, and class.
    pub fn assign_job(env: &Env, required_redundancy: u32) -> soroban_sdk::Vec<Address> {
        let prover_list: soroban_sdk::Vec<Address> = env
            .storage()
            .persistent()
            .get(&DataKey::ProverList)
            .unwrap_or_else(|| soroban_sdk::Vec::new(env));

        let mut assigned = soroban_sdk::Vec::new(env);

        for addr in prover_list.iter() {
            if let Some(p) = Self::get_prover(env, &addr) {
                if !p.is_suspended && p.active_jobs < p.max_concurrency {
                    assigned.push_back(p.prover_address);
                    if assigned.len() == required_redundancy {
                        break;
                    }
                }
            }
        }

        assigned
    }
}
