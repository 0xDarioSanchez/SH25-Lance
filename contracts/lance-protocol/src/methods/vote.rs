use crate::{
    methods::balance::{get_balance, set_balance},
    storage::{
        dispute::{Dispute, get_dispute, set_dispute},
        dispute_status::DisputeStatus,
        error::Error,
        vote::Vote,
        vote::get_anonymous_voting_config,
        voter::get_voter,
    },
};
use soroban_sdk::{
    Address, Bytes, BytesN, Env, U256, Vec, crypto::bls12_381::G1Affine, panic_with_error,
};

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

    // Store the commit hash provided by the judge
    // Judge computes this off-chain as: SHA256(vote_string || secret)
    dispute.voters.push_back(voter);
    dispute.vote_commits.push_back(commit_hash);

    set_dispute(env, dispute_id, dispute.clone());

    Ok(dispute)
}



pub fn reveal_votes(
    env: &Env,
    creator: Address,
    dispute_id: u32,
    votes: soroban_sdk::Vec<bool>,
    secrets: soroban_sdk::Vec<Bytes>,
) -> Result<Dispute, Error> {
    creator.require_auth();

    let mut dispute = get_dispute(env, dispute_id)?;

    // Only dispute creator can reveal votes
    if creator != dispute.creator {
        return Err(Error::NotAuthorized);
    }

    // Check if dispute is resolved
    if dispute.dispute_status == DisputeStatus::FINISHED {
        return Err(Error::DisputeAlreadyResolved);
    }

    // Check if dispute is open
    if dispute.dispute_status != DisputeStatus::OPEN {
        return Err(Error::DisputeNotOpen);
    }

    let commit_count = dispute.voters.len();

    // Validate inputs
    if votes.len() != commit_count || secrets.len() != commit_count {
        return Err(Error::InvalidReveal);
    }

    // Verify all commit hashes and collect valid votes
    for i in 0..commit_count {
        let vote = votes.get(i).unwrap();
        let secret = secrets.get(i).unwrap();
        let stored_commit = dispute.vote_commits.get(i).unwrap();

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

        // Store the vote
        let voter_addr = dispute.voters.get(i).unwrap();
        dispute.votes.push_back(Vote {
            account: voter_addr,
            vote,
        });

        // Update vote counts
        if vote {
            dispute.votes_for += 1;
        } else {
            dispute.votes_against += 1;
        }
    }

    // All votes revealed - resolve the dispute
    dispute.dispute_status = DisputeStatus::FINISHED;
    dispute.finish_timestamp = Some(env.ledger().timestamp());

    // Determine winner and distribute rewards
    const REWARD_PER_CORRECT_VOTE: i128 = 1_000_000; // 0.1 tokens (7 decimals)

    if dispute.votes_for > dispute.votes_against {
        dispute.winner = Some(dispute.creator.clone());

        // Reward voters who voted for the winner
        for i in 0..dispute.voters.len() {
            let vote_val = dispute.votes.get(i).unwrap().vote;
            let voter_addr = dispute.voters.get(i).unwrap();

            if vote_val {
                // Voted for winner - add reward
                let balance = get_balance(env, &voter_addr);
                set_balance(env, &voter_addr, balance + REWARD_PER_CORRECT_VOTE);
            }
        }
    } else {
        dispute.winner = Some(dispute.counterpart.clone());

        // Reward voters who voted for the winner
        for i in 0..dispute.voters.len() {
            let vote_val = dispute.votes.get(i).unwrap().vote;
            let voter_addr = dispute.voters.get(i).unwrap();

            if !vote_val {
                // Voted for winner - add reward
                let balance = get_balance(env, &voter_addr);
                set_balance(env, &voter_addr, balance + REWARD_PER_CORRECT_VOTE);
            }
        }
    }

    set_dispute(env, dispute_id, dispute.clone());

    Ok(dispute)
}

/// Build vote commitments from votes and seeds for anonymous voting.
///
/// Creates BLS12-381 commitments for each vote using the formula:
/// C = g·vote + h·seed where g and h are generator points on BLS12-381.
///
/// Note: This function does not consider voting weights, which are applied
/// during the tallying phase. Calling this on the smart contract would reveal
/// the votes and seeds, so it must be run either in simulation or client-side.
///
/// # Arguments
/// * `env` - The environment object
/// * `project_key` - Unique identifier for the project
/// * `votes` - Vector of vote choices (0=approve, 1=reject, 2=abstain)
/// * `seeds` - Vector of random seeds for each vote
///
/// # Returns
/// * `Vec<BytesN<96>>` - Vector of vote commitments (one per vote)
///
/// # Panics
/// * If no anonymous voting configuration exists for the project
pub fn build_commitments_from_votes(
    env: Env,
    project_id: u32,
    votes: Vec<u128>,
    seeds: Vec<u128>,
) -> Vec<BytesN<96>> {
    // Validate that votes and seeds have the same length
    if votes.len() != seeds.len() {
        panic_with_error!(&env, &Error::TallySeedError);
    }

    let vote_config = get_anonymous_voting_config(env.clone(), project_id);

    let bls12_381 = env.crypto().bls12_381();
    let seed_generator_point = G1Affine::from_bytes(vote_config.seed_generator_point);
    let vote_generator_point = G1Affine::from_bytes(vote_config.vote_generator_point);

    let mut commitments = Vec::new(&env);
    for (vote_, seed_) in votes.iter().zip(seeds.iter()) {
        let vote_: U256 = U256::from_u128(&env, vote_);
        let seed_: U256 = U256::from_u128(&env, seed_);
        let seed_point_ = bls12_381.g1_mul(&seed_generator_point, &seed_.into());
        let vote_point_ = bls12_381.g1_mul(&vote_generator_point, &vote_.into());

        commitments.push_back(bls12_381.g1_add(&vote_point_, &seed_point_).to_bytes());
    }
    commitments
}
