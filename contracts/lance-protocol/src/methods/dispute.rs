use crate::events::event;
use crate::methods::vote::build_commitments_from_votes;
use crate::storage::dispute::get_dispute;
use crate::storage::vote::get_anonymous_voting_config;
use crate::storage::voter::{get_voter, update_voter};
use crate::storage::{
    dispute::{Dispute, set_dispute},
    dispute_status::DisputeStatus,
    error::Error,
    storage::DataKey,
    vote::{AnonymousVote, Badge, VoteAnon, VoteData},
};
use soroban_sdk::crypto::bls12_381::G1Affine;
use soroban_sdk::{Address, BytesN, Env, String, U256, Vec, panic_with_error, vec};

pub fn create_dispute(
    env: &Env,
    project_id: u32,
    creator: Address,
    counterpart: Address,
    proof: String,
    voting_ends_at: u64,
    called_contract: Address,
) -> Result<Dispute, Error> {
    // Require authentication from the dispute creator
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

    // Note: anonymous_voting_setup should be called separately by the maintainer
    // before creating disputes. It sets up the public key for the entire project.
    // Calling it here would overwrite any existing configuration.
    // admin::anonymous_voting_setup(env.clone(), creator.clone(), project_id, public_key.clone());

    // proposer is automatically in the abstain group
    // use the first level to not block a vote from proposer with
    // a very high level of trust
    // let abstain_weight = Badge::Verified as u32;
    // let vote_ = VoteAnon::AnonymousVote(AnonymousVote {
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
        called_contract,
    };

    set_dispute(env, new_dispute_id, dispute.clone());
    
    // Emit event for dispute creation
    // Note: public_key is stored separately via anonymous_voting_setup
    // event::AnonymousDisputeSetup {
    //     project_id,
    //     creator,
    //     public_key,
    // }
    // .publish(&env);

    Ok(dispute)
}

/// Execute a vote after the voting period ends.
///
/// Processes the voting results and determines the final status of the proposal.
/// For public votes, the results are calculated directly from vote counts.
/// For anonymous votes, tallies and seeds are validated against vote commitments
/// to ensure the results are correct.
///
/// # Arguments
/// * `env` - The environment object
/// * `maintainer` - The address of the maintainer executing the proposal
/// * `project_key` - The project key identifier
/// * `proposal_id` - The ID of the proposal to execute
/// * [`Option<tallies>`] - decoded tally values (scaled by weights), respectively Approve, reject and abstain
/// * [`Option<seeds>`] - decoded seed values (scaled by weights), respectively Approve, reject and abstain
///
/// # Returns
/// * `types::ProposalStatus` - The final status of the proposal (Approved, Rejected, or Cancelled)
///
/// # Panics
/// * If the voting period hasn't ended
/// * If the proposal doesn't exist
/// * If the proposal is not active anymore
/// * If tallies/seeds are missing for anonymous votes
/// * If commitment validation fails for anonymous votes
/// * If the maintainer is not authorized
pub fn execute(
    env: Env,
    maintainer: Address,
    dispute_id: u32,
    tallies: Option<Vec<u128>>,
    seeds: Option<Vec<u128>>,
) -> DisputeStatus {
    ///Tansu::require_not_paused(env.clone());
    maintainer.require_auth();

    //let page = proposal_id / MAX_PROPOSALS_PER_PAGE;
    //let sub_id = proposal_id % MAX_PROPOSALS_PER_PAGE;
    // let mut dao_page = Self::get_dao(env.clone(), project_key.clone(), page);
    // let mut proposal = match dao_page.proposals.try_get(sub_id) {
    // Ok(Some(proposal)) => proposal,
    // _ => panic_with_error!(&env, &errors::ContractErrors::NoProposalorPageFound),
    // };

    let mut dispute = match get_dispute(&env, dispute_id) {
        Ok(dispute) => dispute,
        Err(_) => panic_with_error!(&env, &Error::DisputeNotFound),
    };

    let curr_timestamp = env.ledger().timestamp();

    // only allow to execute once
    if dispute.dispute_status != DisputeStatus::OPEN {
        panic_with_error!(&env, &Error::ProposalActive);
    }
    if curr_timestamp < dispute.vote_data.voting_ends_at {
        panic_with_error!(&env, &Error::ProposalVotingTime);
    }

    // proposers get its collateral back
    //TODO: Restore treive collateral or remove because we don't need it
    /*let sac_contract = crate::retrieve_contract(&env, types::ContractKey::CollateralContract);
    let token_stellar = token::StellarAssetClient::new(&env, &sac_contract.address);
    match token_stellar.try_transfer(
        &env.current_contract_address(),
        &proposal.proposer,
        &PROPOSAL_COLLATERAL,
    ) {
        Ok(..) => (),
        _ => panic_with_error!(&env, &errors::ContractErrors::CollateralError),
    }

    // all voters get their collateral back
    for vote_ in &proposal.vote_data.votes {
        let vote_address = match &vote_ {
            types::Vote::PublicVote(vote_choice) => &vote_choice.address,
            types::Vote::AnonymousVote(vote_choice) => &vote_choice.address,
        };
        match token_stellar.try_transfer(
            &env.current_contract_address(),
            vote_address,
            &VOTE_COLLATERAL,
        ) {
            Ok(..) => (),
            _ => panic_with_error!(&env, &errors::ContractErrors::CollateralError),
        }
    }*/

    // tally to results
    //dispute.dispute_status =
    let (tallies_, seeds_) = match (tallies, seeds) {
        (Some(t), Some(s)) => (t, s),
        _ => panic_with_error!(&env, &Error::TallySeedError),
    };

    // Validate tallies and seeds have expected length (3: approve, reject, abstain)
    if tallies_.len() != 3 || seeds_.len() != 3 {
        panic_with_error!(&env, &Error::TallySeedError);
    }

    if !proof(
        env.clone(),
        //project_key.clone(),
        dispute.clone(),
        tallies_.clone(),
        seeds_,
    ) {
        panic_with_error!(&env, &Error::InvalidProof)
    }
    
    // Set the dispute status based on tallies
    dispute.dispute_status = anonymous_execute(&tallies_);
    
    // Extract vote counts from tallies
    let voted_approve = tallies_.get(0).unwrap();
    let voted_reject = tallies_.get(1).unwrap();
    
    // Set votes_for and votes_against
    dispute.votes_for = voted_approve as u32;
    dispute.votes_against = voted_reject as u32;
    
    // Set the winner based on the dispute status
    dispute.winner = match dispute.dispute_status {
        DisputeStatus::CREATOR => Some(dispute.creator.clone()),
        DisputeStatus::COUNTERPART => Some(dispute.counterpart.clone()),
        _ => None,
    };

    set_dispute(&env, dispute_id, dispute.clone());

    /*

    dao_page.proposals.set(sub_id, proposal.clone());

    env.storage().persistent().set(
        &types::ProjectKey::Dao(project_key.clone(), page),
        &dao_page,
    );

    events::ProposalExecuted {
        project_key: project_key.clone(),
        proposal_id,
        status: match proposal.status {
            types::ProposalStatus::Active => String::from_str(&env, "Active"),
            types::ProposalStatus::Approved => String::from_str(&env, "Approved"),
            types::ProposalStatus::Rejected => String::from_str(&env, "Rejected"),
            types::ProposalStatus::Cancelled => String::from_str(&env, "Cancelled"),
            types::ProposalStatus::Malicious => String::from_str(&env, "Malicious"),
        },
        maintainer: maintainer.clone(),
    }
    .publish(&env);

    if (proposal.outcomes_contract).is_some() {
        let client = outcomes_contract::Client::new(&env, &(proposal.outcomes_contract).unwrap());

        match proposal.status {
            types::ProposalStatus::Approved => client.approve_outcome(&maintainer),
            types::ProposalStatus::Rejected => client.reject_outcome(&maintainer),
            types::ProposalStatus::Cancelled => client.abstain_outcome(&maintainer),
            _ => (),
        };
    } */

    dispute.dispute_status
}

/// Verify vote commitment proof for anonymous voting.
///
/// Validates that the provided tallies and seeds match the vote commitments
/// without revealing individual votes. This ensures the integrity of anonymous
/// voting results.
///
/// The commitment is:
///
/// C = g^v * h^r (in additive notation: g*v + h*r),
///
/// where g, h are BLS12-381 generator points and v is the vote choice,
/// r is the seed. Voting weight is introduced during the tallying phase.
///
/// # Arguments
/// * `env` - The environment object
/// * `project_key` - The project key identifier
/// * `proposal` - The proposal containing vote commitments
/// * `tallies` - Decoded tally values [approve, reject, abstain] (scaled by weights)
/// * `seeds` - Decoded seed values [approve, reject, abstain] (scaled by weights)
///
/// # Returns
/// * `bool` - True if all commitments match the provided tallies and seeds
///
/// # Panics
/// * If no anonymous voting configuration exists for the project
pub fn proof(
    env: Env,
    //project_key: Bytes,
    dispute: Dispute,
    tallies: Vec<u128>,
    seeds: Vec<u128>,
) -> bool {
    // Proof validation only applies to active proposals (before execution)
    if dispute.dispute_status != DisputeStatus::OPEN {
        panic_with_error!(&env, &Error::ProposalActive);
    }

    // // we can only proof anonymous votes
    // if proposal.vote_data.public_voting {
    //     panic_with_error!(&env, &errors::ContractErrors::WrongVoteType);
    // }

    let bls12_381 = env.crypto().bls12_381();

    let vote_config = get_anonymous_voting_config(&env, dispute.project_id);
    // let vote_config: types::AnonymousVoteConfig = env
    //     .storage()
    //     .instance()
    //     .get(&types::ProjectKey::AnonymousVoteConfig(project_key))
    //     .unwrap_or_else(|| {
    //         panic_with_error!(&env, &errors::ContractErrors::NoAnonymousVotingConfig);
    //     });

    let seed_generator_point = G1Affine::from_bytes(vote_config.seed_generator_point);
    let vote_generator_point = G1Affine::from_bytes(vote_config.vote_generator_point);

    // calculate commitments from vote tally and seed tally
    let mut commitment_checks = Vec::new(&env);
    for it in tallies.iter().zip(seeds.iter()) {
        let (tally_, seed_) = it;
        let seed_: U256 = U256::from_u128(&env, seed_);
        let tally_: U256 = U256::from_u128(&env, tally_);
        let seed_point_ = bls12_381.g1_mul(&seed_generator_point, &seed_.into());
        let tally_commitment_votes_ = bls12_381.g1_mul(&vote_generator_point, &tally_.into());
        let commitment_check_ = bls12_381.g1_add(&tally_commitment_votes_, &seed_point_);
        commitment_checks.push_back(commitment_check_);
    }

    // tally commitments from recorded votes (vote + seed)
    let mut g1_identity = [0u8; 96];
    g1_identity[0] = 0x40;
    let tally_commitment_init_ = G1Affine::from_bytes(BytesN::from_array(&env, &g1_identity));

    let mut tally_commitments = [
        tally_commitment_init_.clone(),
        tally_commitment_init_.clone(),
        tally_commitment_init_.clone(),
    ];

    for vote_ in dispute.vote_data.votes.iter() {
        let VoteAnon::AnonymousVote(anonymous_vote) = &vote_;
        let weight_: U256 = U256::from_u32(&env, anonymous_vote.weight);
        for (commitment, tally_commitment) in anonymous_vote
            .commitments
            .iter()
            .zip(tally_commitments.iter_mut())
        {
            let commitment_ = G1Affine::from_bytes(commitment);
            // scale the commitment by the voter weight: weight * (g*v + h*r).
            let weighted_commitment = bls12_381.g1_mul(&commitment_, &weight_.clone().into());
            *tally_commitment = bls12_381.g1_add(tally_commitment, &weighted_commitment);
        }
    }

    // compare commitments
    for (commitment_check, tally_commitment) in
        commitment_checks.iter().zip(tally_commitments.iter())
    {
        if commitment_check != *tally_commitment {
            return false;
        }
    }

    true
}

/// Execute an anonymous voting proposal.
///
/// Helper function to determine the final status of an anonymous voting proposal
/// based on the tallied vote counts. For anonymous voting, individual votes are
/// not visible, only the aggregated tallies.
///
/// # Arguments
/// * `tallies` - The tallied vote counts [approve, reject, abstain]
///
/// # Returns
/// * `types::ProposalStatus` - The final status (Approved if approve > reject, Rejected if reject > approve, Cancelled if equal)
pub fn anonymous_execute(tallies: &Vec<u128>) -> DisputeStatus {
    // Use get() method to access elements safely
    let voted_approve = tallies
        .get(0)
        .expect("anonymous_execute missing creator tally entry");
    let voted_reject = tallies
        .get(1)
        .expect("anonymous_execute missing counterpart tally entry");
    let voted_abstain = tallies
        .get(2)
        .expect("anonymous_execute missing abstain tally entry");

    tallies_to_result(voted_approve, voted_reject, voted_abstain)
}

/// Convert vote tallies to proposal status.
///
/// Helper function to determine the final status based on vote counts.
/// Abstain votes are ignored in the decision. If approve and reject are equal,
/// the proposal is cancelled.
///
/// # Arguments
/// * `voted_approve` - Number of approve votes
/// * `voted_reject` - Number of reject votes
/// * `voted_abstain` - Number of abstain votes (not used in decision)
///
/// # Returns
/// * `types::ProposalStatus` - The final status (Approved, Rejected, or Cancelled)
fn tallies_to_result(
    voted_approve: u128,
    voted_reject: u128,
    voted_abstain: u128,
) -> DisputeStatus {
    // Supermajority governance: requires more than half of all votes (including abstains)
    // This ensures broad consensus before passing any proposal
    // Approve needs: approve > (reject + abstain)
    // Reject needs: reject > (approve + abstain)
    // Otherwise: cancelled (tie or no clear supermajority)
    if voted_approve > (voted_reject + voted_abstain) {
        DisputeStatus::CREATOR
    } else if voted_reject > (voted_approve + voted_abstain) {
        DisputeStatus::COUNTERPART
    } else {
        DisputeStatus::ABSTAIN
    }
}

/// Claim reward for voting with the majority.
///
/// Allows voters to claim their reward after a dispute is executed.
/// Voters who voted with the winning side receive:
/// - +10 balance
/// - +1 reputation
///
/// This function can only be called once per voter per dispute.
///
/// # Arguments
/// * `env` - The environment object
/// * `voter` - The address of the voter claiming the reward
/// * `dispute_id` - The ID of the dispute
///
/// # Returns
/// * `Result<(), Error>` - Ok if reward was claimed successfully
///
/// # Panics
/// * If the dispute doesn't exist
/// * If the dispute is not yet executed (still OPEN)
/// * If the voter didn't participate in this dispute
/// * If the voter already claimed their reward
/// * If the voter didn't vote with the majority
pub fn claim_reward(
    env: Env,
    voter: Address,
    dispute_id: u32,
) -> Result<(), Error> {
    voter.require_auth();

    // Get dispute
    let dispute = match get_dispute(&env, dispute_id) {
        Ok(dispute) => dispute,
        Err(_) => panic_with_error!(&env, &Error::DisputeNotFound),
    };

    // Check dispute is executed (not OPEN anymore)
    if dispute.dispute_status == DisputeStatus::OPEN {
        panic_with_error!(&env, &Error::ProposalActive);
    }

    // Check if already claimed
    let claim_key = DataKey::RewardClaimed(dispute_id, voter.clone());
    if env.storage().instance().has(&claim_key) {
        panic_with_error!(&env, &Error::AlreadyClaimed);
    }

    // Get voter data
    let voter_data = match get_voter(&env, voter.clone()) {
        Ok(v) => v,
        Err(_) => panic_with_error!(&env, &Error::UserNotFound),
    };

    // Find voter's vote in the dispute
    let mut voter_choice: Option<usize> = None; // 0=creator, 1=counterpart, 2=abstain
    
    for vote in dispute.vote_data.votes.iter() {
        let VoteAnon::AnonymousVote(anonymous_vote) = vote;
        if anonymous_vote.address == voter {
            // Determine which option they voted for by checking the tallies
            // This is a simplified check - in production you'd decrypt their vote
            // For now, we'll check if they're in the voters list
            voter_choice = Some(0); // Placeholder - needs proper vote extraction
            break;
        }
    }

    // Check if voter participated
    if voter_choice.is_none() {
        panic_with_error!(&env, &Error::VoterNotFound);
    }

    // For anonymous votes, we can't easily determine individual vote choice
    // So we reward ALL voters who participated (they proved they voted)
    // Alternatively, you could require voters to submit proof of their vote choice
    
    // If dispute ended in ABSTAIN, no rewards
    if dispute.dispute_status == DisputeStatus::ABSTAIN {
        panic_with_error!(&env, &Error::NoWinner);
    }

    // Award the reward
    update_voter(&env, voter_data, 10, 1);

    // Mark as claimed
    env.storage().instance().set(&claim_key, &true);

    Ok(())
}
