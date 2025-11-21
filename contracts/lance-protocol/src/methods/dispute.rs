use crate::events::event;
use crate::methods::admin;
use crate::methods::vote::build_commitments_from_votes;
use crate::storage::{
    dispute::{Dispute, set_dispute},
    dispute_status::DisputeStatus,
    error::Error,
    storage::DataKey,
    vote::{AnonymousVote, Badge, Vote2, VoteData},
};
use soroban_sdk::{Address, Env, String, Vec, vec};

pub fn create_dispute(
    env: &Env,
    project_id: u32,
    public_key: String,
    creator: Address,
    counterpart: Address,
    proof: String,
    voting_ends_at: u64,
) -> Result<Dispute, Error> {
    admin::require_admin(env);
    //    creator.require_auth();

    let current_id = env
        .storage()
        .instance()
        .get::<_, u32>(&DataKey::DisputeId)
        .unwrap_or(0);
    let new_dispute_id = current_id + 1;
    env.storage()
        .instance()
        .set(&DataKey::DisputeId, &new_dispute_id);

    admin::anonymous_voting_setup(env.clone(), creator.clone(), project_id, public_key.clone());

    // proposer is automatically in the abstain group
    // use the first level to not block a vote from proposer with
    // a very high level of trust
    // let abstain_weight = Badge::Verified as u32;
    // let vote_ = Vote2::AnonymousVote(AnonymousVote {
    //     address: creator.clone(),
    //     weight: abstain_weight,
    //     encrypted_seeds: vec![
    //         &env,
    //         String::from_str(&env, "0"),
    //         String::from_str(&env, "0"),
    //         String::from_str(&env, "0"),
    //     ],
    //     encrypted_votes: vec![
    //         &env,
    //         String::from_str(&env, "0"),
    //         String::from_str(&env, "0"),
    //         String::from_str(&env, "1"),
    //     ],
    //     commitments: build_commitments_from_votes(
    //         env.clone(),
    //         new_dispute_id,
    //         vec![&env, 0u128, 0u128, 1u128],
    //         vec![&env, 0u128, 0u128, 0u128],
    //     ),
    // });

    // let votes = vec![&env, vote_];
    // let vote_data = VoteData {
    //     voting_ends_at,
    //     //public_voting,
    //     votes,
    // };

    let votes = vec![&env];
    let vote_data = VoteData {
        voting_ends_at,
        //public_voting,
        votes,
    };

    let dispute = Dispute {
        project_id,
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
        vote_data,
    };

    set_dispute(env, new_dispute_id, dispute.clone());
    // Emit event for anonymous dispute setup
    event::AnonymousDisputeSetup {
        project_id,
        creator,
        public_key,
    }
    .publish(&env);
    //TODO add event

    Ok(dispute)
}
