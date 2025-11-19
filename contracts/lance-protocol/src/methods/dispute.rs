use crate::{ storage::{
    dispute::{set_dispute, Dispute},
    dispute_status::DisputeStatus,
    error::Error,
    storage::DataKey,
}};
use soroban_sdk::{Address, Env, String, Vec};

pub fn create_dispute(
    env: &Env,
    creator: Address,
    counterpart: Address,
    proof: String,
) -> Result<Dispute, Error> {
    creator.require_auth();

    let current_id = env
        .storage()
        .instance()
        .get::<_, u32>(&DataKey::DisputeId)
        .unwrap_or(0);
    let new_dispute_id = current_id + 1;
    env.storage()
        .instance()
        .set(&DataKey::DisputeId, &new_dispute_id);

    let dispute = Dispute {
        dispute_id: new_dispute_id,
        able_to_vote: Vec::new(env),
        voters: Vec::new(env),
        vote_commits: Vec::new(env),
        votes: Vec::new(env),
        dispute_status: DisputeStatus::OPEN,
        initial_timestamp: env.ledger().timestamp(),
        finish_timestamp: None,
        creator: creator.clone(),
        counterpart,
        winner: None,
        creator_proves: proof.clone(),      
        counterpart_proves: None,
        waiting_for_judges: false,
        votes_for: 0,
        votes_against: 0,
        //TODO payment: 0,
    };

    set_dispute(env, new_dispute_id, dispute.clone());

    //TODO add event
        
    Ok(dispute)
}