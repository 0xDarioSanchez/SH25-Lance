use soroban_sdk::{contracttype, Address, String, Vec, Env, BytesN};
use super::{dispute_status::DisputeStatus, vote::Vote};
use crate::storage::{error::Error, storage::DataKey};


#[derive(Clone)]
#[contracttype]
pub struct Dispute {
    pub dispute_id: u32,
    pub able_to_vote: Vec<Address>,        // Judges who can vote
    pub voters: Vec<Address>,              // Judges who have committed
    pub vote_commits: Vec<BytesN<32>>,   // Commit hashes
    pub votes: Vec<Vote>,                  // Revealed votes
    pub dispute_status: DisputeStatus,
    pub initial_timestamp: u64,
    pub finish_timestamp: Option<u64>,
    pub creator: Address,
    pub counterpart: Address,  
    pub winner: Option<Address>,
    pub creator_proves: String,      
    pub counterpart_proves: Option<String>,
    pub waiting_for_judges: bool,
    pub votes_for: u32,
    pub votes_against: u32,
    //TODO add payment: i128, ???
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