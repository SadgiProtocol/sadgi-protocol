#![no_std]

pub mod escrow;
pub mod queue;
pub mod scheduler;
pub mod settlement;

use sadgi_types::events::ProtocolEvent;
use sadgi_types::receipt::ProofReceipt;
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Bytes, BytesN, Env, IntoVal};

#[contract]
pub struct SadgiMarketplace;

#[contractimpl]
impl SadgiMarketplace {
    /// Developers call this to request a zero-knowledge proof with dynamic classes and redundancy.
    pub fn create_job(
        env: Env,
        developer: Address,
        class: queue::JobClass,
        bounty: i128,
        redundancy: u32,
    ) -> u64 {
        developer.require_auth();

        // Anti-Griefing: Minimum Escrow Check based on Job Class
        let min_bounty = match class {
            queue::JobClass::Standard => 10,
            queue::JobClass::Priority => 50,
            queue::JobClass::Bulk => 5,
            queue::JobClass::AI => 200,
        };
        if bounty < min_bounty {
            panic!("Bounty below minimum threshold for this job class");
        }

        let job_id = env.ledger().sequence() as u64; // Simple ID for mockup

        // Lock funds in escrow
        escrow::Escrow::lock_funds(&env, developer.clone(), bounty);

        let current_ledger = env.ledger().sequence();

        // Push to queue as Pending
        queue::Queue::push(
            &env,
            queue::JobRequest {
                job_id,
                developer: developer.clone(),
                class,
                bounty,
                redundancy,
                state: queue::JobState::Pending,
                assignment_deadline: current_ledger + 100,
                acceptance_deadline: current_ledger + 200,
                submission_deadline: current_ledger + 1000,
                assigned_provers: soroban_sdk::Vec::new(&env),
            },
        );

        env.events().publish(
            (soroban_sdk::symbol_short!("job_new"), job_id),
            ProtocolEvent::JobCreated {
                job_id,
                developer,
                program_id: soroban_sdk::BytesN::from_array(&env, &[0; 32]),
                bounty,
            },
        );

        job_id
    }

    /// Cron-like function to match queued jobs to capable provers.
    pub fn assign_jobs(env: Env, job_id: u64) {
        let mut job = queue::Queue::get_job(&env, job_id).expect("Job not found");

        if job.state != queue::JobState::Pending {
            panic!("Job not pending");
        }

        let assigned = scheduler::Scheduler::assign_job(&env, job.redundancy);
        if assigned.is_empty() {
            // No provers available, stays pending.
            return;
        }

        job.assigned_provers = assigned;
        job.state = queue::JobState::Assigned;
        queue::Queue::update_job_state(&env, job_id, queue::JobState::Assigned);
    }

    /// Provers call this to submit their final cryptographic receipt.
    pub fn submit_proof(
        env: Env,
        prover: Address,
        job_id: u64,
        receipt: ProofReceipt,
        registry_id: Address,
        verifier_id: Address,
    ) {
        prover.require_auth();

        let job = queue::Queue::get_job(&env, job_id).expect("Job not found");

        if job.state == queue::JobState::Settled || job.state == queue::JobState::Verified {
            panic!("Job already completed by another prover");
        }

        // Verify deadlines
        if env.ledger().sequence() > job.submission_deadline {
            // Initiate progressive penalty for timeout
            settlement::Settlement::penalize_prover(&env, prover, false);
            queue::Queue::update_job_state(&env, job_id, queue::JobState::Failed);
            return;
        }

        // 1. Fetch Verification Key from Registry
        // (In a real implementation, we'd use the strongly typed client, but here we use invoke_contract for decoupling)
        // Let's assume the registry returns a tuple of (vk, version, metadata) or a custom type.
        // For simplicity in this architecture demo, we'll assume the registry returns the `vk` directly if we call `get_vk`.
        let vk: Bytes = env.invoke_contract(
            &registry_id,
            &symbol_short!("get_vk"),
            (receipt.program_id.clone(),).into_val(&env),
        );

        // 2. Call Verifier Contract
        let is_valid: bool = env.invoke_contract(
            &verifier_id,
            &symbol_short!("verify"),
            (receipt.proof.clone(), receipt.public_values.clone(), vk).into_val(&env),
        );

        // Cryptographic Verification
        if is_valid {
            // Fastest valid proof wins! Release funds (5% marketplace fee)
            settlement::Settlement::release_funds(&env, prover.clone(), job.bounty, 5);
            queue::Queue::update_job_state(&env, job_id, queue::JobState::Settled);

            env.events().publish(
                (soroban_sdk::symbol_short!("job_done"), job_id),
                ProtocolEvent::ProofVerified { job_id },
            );
        } else {
            // Cryptographic Fraud detected. Maximum Penalty!
            settlement::Settlement::penalize_prover(&env, prover, true);

            env.events().publish(
                (soroban_sdk::symbol_short!("job_fail"), job_id),
                ProtocolEvent::JobFailed {
                    job_id,
                    reason: soroban_sdk::symbol_short!("invalid"),
                },
            );
        }
    }
}

mod test;
