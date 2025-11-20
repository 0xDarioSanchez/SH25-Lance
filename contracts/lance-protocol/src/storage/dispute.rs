use soroban_sdk::{contracttype, Address, String, Vec, Env, BytesN};
use super::{dispute_status::DisputeStatus, vote::Vote};
use crate::storage::{error::Error, storage::DataKey};


#[derive(Clone)]
#[contracttype]
pub struct Dispute {
    pub dispute_id: u32,
    pub contract_address: Address,
    pub requester: Address,
    pub beneficiary: Address,
    pub voters: Vec<Address>,        // ordered list of judges for this dispute
    pub commits: Vec<BytesN<32>>,    // commits per voter index
    pub revealed: Vec<bool>,         // whether each vote is revealed
    pub vote_plain: Vec<bool>,       // revealed vote booleans
    pub commits_count: u32,
    pub reveals_count: u32,
    pub votes_for: u32,
    pub votes_against: u32,
    pub waiting_for_judges: bool,
    pub is_open: bool,
    pub resolved: bool,
}

pub(crate) fn get_dispute(env: &Env, dispute_id: u32) -> Result<Dispute, Error> {
    let key = DataKey::Disputes(dispute_id);

    env.storage()
        .instance()
        .get(&key)
        .ok_or(Error::DisputeNotFound)
}

pub(crate) fn set_dispute(env: &Env, dispute_id: u32, dispute: Dispute) {
    let key = DataKey::Disputes(dispute_id);

    env.storage().instance().set(&key, &dispute)
}