use crate::{
    methods::balance::{get_balance, set_balance},
    storage::{
        dispute::{get_dispute, set_dispute, Dispute},
        dispute_status::DisputeStatus,
        error::Error,
        voter::get_voter,
        vote::Vote,
    },
};
use soroban_sdk::{Address, Env, BytesN, Bytes};

const VOTE_BASE_POWER: u32 = 1;
const MIN_VOTES_TO_FINISH_DISPUTE: u32 = 5;


pub fn register_to_vote(env: &Env, voter: Address, dispute_id: u32) -> Result<Dispute, Error> {
    voter.require_auth();

    let _new_judge_caller = get_voter(env, voter.clone())?;

    let mut dispute = get_dispute(env, dispute_id)?;

    if dispute.dispute_status != DisputeStatus::OPEN {
        return Err(Error::InvalidDisputeStatus);
    }

    if voter == dispute.creator || voter == dispute.counterpart {
        return Err(Error::NotAuthorized);
    }

    // Check if voter is not already registered
    for able_voter in dispute.able_to_vote.iter() {
        if able_voter == voter {
            return Err(Error::JudgeAlreadyVoted);
        }
    }

    dispute.able_to_vote.push_back(voter);
    set_dispute(env, dispute_id, dispute.clone());

    Ok(dispute)
}
