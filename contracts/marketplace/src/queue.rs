#![allow(clippy::manual_find)]
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

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Job(u64),
}

pub struct Queue;

impl Queue {
    pub fn push(env: &Env, request: JobRequest) {
        env.storage()
            .persistent()
            .set(&DataKey::Job(request.job_id), &request);
    }

    pub fn get_job(env: &Env, job_id: u64) -> Option<JobRequest> {
        env.storage().persistent().get(&DataKey::Job(job_id))
    }

    pub fn update_job_state(env: &Env, job_id: u64, new_state: JobState) {
        if let Some(mut job) = Self::get_job(env, job_id) {
            job.state = new_state;
            env.storage().persistent().set(&DataKey::Job(job_id), &job);
        }
    }
}
