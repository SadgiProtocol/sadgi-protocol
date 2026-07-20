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
        let key = soroban_sdk::symbol_short!("provers");
        let mut provers: soroban_sdk::Vec<ProverProfile> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| soroban_sdk::Vec::new(env));

        provers.push_back(profile);
        env.storage().persistent().set(&key, &provers);
    }

    pub fn get_prover(env: &Env, prover_address: &Address) -> Option<ProverProfile> {
        let key = soroban_sdk::symbol_short!("provers");
        let provers: soroban_sdk::Vec<ProverProfile> = env.storage().persistent().get(&key)?;

        for p in provers.iter() {
            if p.prover_address == *prover_address {
                return Some(p);
            }
        }
        None
    }

    pub fn update_prover(env: &Env, prover_address: &Address, updated_profile: ProverProfile) {
        let key = soroban_sdk::symbol_short!("provers");
        let mut provers: soroban_sdk::Vec<ProverProfile> =
            env.storage().persistent().get(&key).unwrap();

        for (i, p) in provers.iter().enumerate() {
            if p.prover_address == *prover_address {
                provers.set(i as u32, updated_profile);
                break;
            }
        }

        env.storage().persistent().set(&key, &provers);
    }

    /// Select the best Provers based on capacity, reputation, and class.
    pub fn assign_job(env: &Env, required_redundancy: u32) -> soroban_sdk::Vec<Address> {
        // Mocking assignment logic: select first N available provers not suspended
        let key = soroban_sdk::symbol_short!("provers");
        let provers: soroban_sdk::Vec<ProverProfile> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| soroban_sdk::Vec::new(env));

        let mut assigned = soroban_sdk::Vec::new(env);

        for p in provers.iter() {
            if !p.is_suspended && p.active_jobs < p.max_concurrency {
                assigned.push_back(p.prover_address);
                if assigned.len() == required_redundancy {
                    break;
                }
            }
        }

        assigned
    }
}
