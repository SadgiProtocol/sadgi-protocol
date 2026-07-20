use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JobClass {
    Standard,
    Priority,
    Bulk,
    AI,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JobState {
    Pending,
    Queued,
    Assigned,
    Accepted,
    Computing,
    Submitted,
    Verified,
    Settled,
    Failed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JobRequest {
    pub job_id: u64,
    pub developer: Address,
    pub class: JobClass,
    pub bounty: i128,
    pub redundancy: u32, // How many provers can compute this concurrently (fastest wins)
    pub state: JobState,

    // Deadlines (tracked in ledger sequence numbers)
    pub assignment_deadline: u32,
    pub acceptance_deadline: u32,
    pub submission_deadline: u32,

    pub assigned_provers: soroban_sdk::Vec<Address>,
}

pub struct Queue;

impl Queue {
    pub fn push(env: &Env, request: JobRequest) {
        let key = soroban_sdk::symbol_short!("queue");
        let mut queue: soroban_sdk::Vec<JobRequest> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| soroban_sdk::Vec::new(env));

        queue.push_back(request);
        env.storage().persistent().set(&key, &queue);
    }

    pub fn get_job(env: &Env, job_id: u64) -> Option<JobRequest> {
        let key = soroban_sdk::symbol_short!("queue");
        let queue: soroban_sdk::Vec<JobRequest> = env.storage().persistent().get(&key)?;

        for job in queue.iter() {
            if job.job_id == job_id {
                return Some(job);
            }
        }
        None
    }

    pub fn update_job_state(env: &Env, job_id: u64, new_state: JobState) {
        let key = soroban_sdk::symbol_short!("queue");
        let mut queue: soroban_sdk::Vec<JobRequest> = env.storage().persistent().get(&key).unwrap();

        for (i, mut job) in queue.iter().enumerate() {
            if job.job_id == job_id {
                job.state = new_state.clone();
                queue.set(i as u32, job);
                break;
            }
        }

        env.storage().persistent().set(&key, &queue);
    }
}
