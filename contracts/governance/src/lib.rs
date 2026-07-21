#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Symbol, Val, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    IsCouncilMember(Address),
    CouncilSize,
    ProposalCount,
    Proposal(u32),
    HasApproved(u32, Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub id: u32,
    pub proposer: Address,
    pub target_contract: Address,
    pub action: Symbol,
    pub args: Vec<Val>,
    pub approvals: u32,
    pub executed: bool,
    pub expiration_ledger: u32,
}

#[contract]
pub struct SadgiAdministration;

#[contractimpl]
impl SadgiAdministration {
    /// Initialize the Administration contract with an initial council.
    pub fn init(env: Env, initial_council: Vec<Address>) {
        if env.storage().instance().has(&DataKey::CouncilSize) {
            panic!("Already initialized");
        }

        let size = initial_council.len();
        if size == 0 {
            panic!("Council must have at least 1 member");
        }

        for member in initial_council.iter() {
            env.storage()
                .instance()
                .set(&DataKey::IsCouncilMember(member), &true);
        }

        env.storage().instance().set(&DataKey::CouncilSize, &size);
        env.storage().instance().set(&DataKey::ProposalCount, &0u32);
    }

    /// Submit a new administrative proposal. Only Council members can propose.
    pub fn propose(
        env: Env,
        proposer: Address,
        target_contract: Address,
        action: Symbol,
        args: Vec<Val>,
        lifetime_ledgers: u32,
    ) -> u32 {
        proposer.require_auth();

        if !env
            .storage()
            .instance()
            .get(&DataKey::IsCouncilMember(proposer.clone()))
            .unwrap_or(false)
        {
            panic!("Not a council member");
        }

        let mut count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0);
        count += 1;

        let proposal = Proposal {
            id: count,
            proposer: proposer.clone(),
            target_contract,
            action,
            args,
            approvals: 1, // Proposer implicitly approves
            executed: false,
            expiration_ledger: env.ledger().sequence() + lifetime_ledgers,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Proposal(count), &proposal);
        env.storage()
            .persistent()
            .set(&DataKey::HasApproved(count, proposer), &true);
        env.storage()
            .instance()
            .set(&DataKey::ProposalCount, &count);

        count
    }

    /// Approve an active proposal.
    pub fn approve(env: Env, council_member: Address, proposal_id: u32) {
        council_member.require_auth();

        if !env
            .storage()
            .instance()
            .get(&DataKey::IsCouncilMember(council_member.clone()))
            .unwrap_or(false)
        {
            panic!("Not a council member");
        }

        if env
            .storage()
            .persistent()
            .has(&DataKey::HasApproved(proposal_id, council_member.clone()))
        {
            panic!("Already approved");
        }

        let mut proposal: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .expect("Proposal not found");

        if proposal.executed {
            panic!("Proposal already executed");
        }

        if env.ledger().sequence() > proposal.expiration_ledger {
            panic!("Proposal expired");
        }

        proposal.approvals += 1;

        env.storage()
            .persistent()
            .set(&DataKey::Proposal(proposal_id), &proposal);
        env.storage()
            .persistent()
            .set(&DataKey::HasApproved(proposal_id, council_member), &true);
    }

    /// Execute a proposal if it has reached the M-of-N threshold.
    pub fn execute(env: Env, caller: Address, proposal_id: u32) {
        caller.require_auth();

        let mut proposal: Proposal = env
            .storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .expect("Proposal not found");

        if proposal.executed {
            panic!("Already executed");
        }

        if env.ledger().sequence() > proposal.expiration_ledger {
            panic!("Proposal expired");
        }

        let council_size: u32 = env
            .storage()
            .instance()
            .get(&DataKey::CouncilSize)
            .unwrap_or(0);

        // M-of-N Threshold (e.g. 51% majority)
        let threshold = (council_size / 2) + 1;

        if proposal.approvals < threshold {
            panic!("Proposal has not reached approval threshold");
        }

        // Execute the cross-contract call.
        // The Administration contract must be the Admin of the target contract.
        let _res: Val = env.invoke_contract(
            &proposal.target_contract,
            &proposal.action,
            proposal.args.clone(),
        );

        proposal.executed = true;
        env.storage()
            .persistent()
            .set(&DataKey::Proposal(proposal_id), &proposal);
    }

    /// Read-only: Get a proposal's current state
    pub fn get_proposal(env: Env, proposal_id: u32) -> Proposal {
        env.storage()
            .persistent()
            .get(&DataKey::Proposal(proposal_id))
            .expect("Proposal not found")
    }
}
