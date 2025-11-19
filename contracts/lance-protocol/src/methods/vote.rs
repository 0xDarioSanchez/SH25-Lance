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

//TODO add
// const VOTE_BASE_POWER: u32 = 1;
// const MIN_VOTES_TO_FINISH_DISPUTE: u32 = 5;


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



pub fn commit_vote(
    env: &Env,
    voter: Address,
    dispute_id: u32,
    commit_hash: BytesN<32>,
) -> Result<Dispute, Error> {
    voter.require_auth();

    let mut dispute = get_dispute(env, dispute_id)?;

    // Check if dispute is resolved
    if dispute.dispute_status == DisputeStatus::FINISHED {
        return Err(Error::DisputeAlreadyResolved);
    }

    // Check if dispute is open
    if dispute.dispute_status != DisputeStatus::OPEN {
        return Err(Error::DisputeNotOpen);
    }

    // Check if judge is allowed to vote
    let mut allowed = false;
    for able_voter in dispute.able_to_vote.iter() {
        if able_voter == voter {
            allowed = true;
            break;
        }
    }
    if !allowed {
        return Err(Error::JudgeNotAllowedToVote);
    }

    // Check if judge has not already committed
    for committed_voter in dispute.voters.iter() {
        if committed_voter == voter {
            return Err(Error::JudgeAlreadyVoted);
        }
    }

    // Store commit
    dispute.voters.push_back(voter);
    dispute.vote_commits.push_back(commit_hash);
    
    set_dispute(env, dispute_id, dispute.clone());

    Ok(dispute)
}

pub fn reveal_vote(
    env: &Env,
    voter: Address,
    dispute_id: u32,
    vote: bool,
    secret: Bytes,
) -> Result<Dispute, Error> {
    voter.require_auth();

    let mut dispute = get_dispute(env, dispute_id)?;

    // Check if dispute is resolved
    if dispute.dispute_status == DisputeStatus::FINISHED {
        return Err(Error::DisputeAlreadyResolved);
    }

    // Find the judge's commit index
    let mut judge_index: Option<u32> = None;
    let commit_count = dispute.voters.len();
    
    for i in 0..commit_count {
        let committed_voter = dispute.voters.get(i).unwrap();
        if committed_voter == voter {
            judge_index = Some(i);
            break;
        }
    }

    let idx = match judge_index {
        Some(i) => i,
        None => return Err(Error::JudgeNotCommitted),
    };

    // Check if already revealed
    if dispute.revealed.get(idx).unwrap_or(false) {
        return Err(Error::JudgeAlreadyVoted);
    }

    // Verify the commit hash
    let stored_commit = dispute.vote_commits.get(idx).unwrap();
    
    // Compute hash(vote_string || secret)
    let vote_str = if vote { "true" } else { "false" };
    let mut data = Bytes::new(env);
    data.append(&Bytes::from_slice(env, vote_str.as_bytes()));
    data.append(&secret);
    
    let computed_hash: BytesN<32> = env.crypto().sha256(&data).into();

    // Verify hash matches
    if stored_commit != computed_hash {
        return Err(Error::InvalidReveal);
    }

    // Mark as revealed and store the vote
    dispute.revealed.set(idx, true);
    dispute.votes.push_back(Vote {
        account: voter.clone(),
        vote,
    });

    // Update vote counts
    if vote {
        dispute.votes_for += 1;
    } else {
        dispute.votes_against += 1;
    }

    // Check if all votes are revealed
    let total_reveals = dispute.revealed.iter().filter(|&r| r).count();
    let required_votes = dispute.able_to_vote.len();
    
    if total_reveals == required_votes as usize {
        // All votes revealed - resolve the dispute
        dispute.dispute_status = DisputeStatus::FINISHED;
        dispute.finish_timestamp = Some(env.ledger().timestamp());

        // Determine winner
        if dispute.votes_for > dispute.votes_against {
            dispute.winner = Some(dispute.creator.clone());
            
            // Update balances and reputation for voters
            for i in 0..dispute.voters.len() {
                let vote_val = dispute.votes.get(i).unwrap().vote;
                let voter_addr = dispute.voters.get(i).unwrap();
                
                if vote_val {
                    // Voted for winner
                    let balance = get_balance(env, &voter_addr);
                    // Prize distribution logic would go here
                    set_balance(env, &voter_addr, balance);
                }
            }
        } else {
            dispute.winner = Some(dispute.counterpart.clone());
            
            // Update balances and reputation for voters
            for i in 0..dispute.voters.len() {
                let vote_val = dispute.votes.get(i).unwrap().vote;
                let voter_addr = dispute.voters.get(i).unwrap();
                
                if !vote_val {
                    // Voted for winner
                    let balance = get_balance(env, &voter_addr);
                    // Prize distribution logic would go here
                    set_balance(env, &voter_addr, balance);
                }
            }
        }
    }

    set_dispute(env, dispute_id, dispute.clone());

    Ok(dispute)
}
