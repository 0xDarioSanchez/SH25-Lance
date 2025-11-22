use soroban_sdk::{
    String,
    testutils::Ledger,
    vec,
};

use crate::{
    storage::dispute_status::DisputeStatus,
    storage::vote::{AnonymousVote, VoteAnon},
    tests::test_utils::{create_test_data, init_contract},
};

#[test]
fn test_claim_reward_success() {
    let setup = create_test_data();

    let public_key = String::from_str(&setup.env, "public key random");
    setup
        .contract
        .anonymous_voting_setup(&setup.contract_admin, &setup.project_id, &public_key);

    let voting_ends_at = setup.env.ledger().timestamp() + 3600 * 24 * 2;
    let dispute = init_contract(&setup);

    // Judge1 votes
    let vote_ = VoteAnon::AnonymousVote(AnonymousVote {
        address: setup.judge1.clone(),
        weight: 3,
        encrypted_seeds: vec![
            &setup.env,
            String::from_str(&setup.env, "fafdas"),
            String::from_str(&setup.env, "fafdas"),
            String::from_str(&setup.env, "fafdas"),
        ],
        encrypted_votes: vec![
            &setup.env,
            String::from_str(&setup.env, "fafdas"),
            String::from_str(&setup.env, "fafdas"),
            String::from_str(&setup.env, "rewrewr"),
        ],
        commitments: setup.contract.build_commitments_from_votes(
            &dispute.dispute_id,
            &vec![&setup.env, 3u128, 1u128, 1u128],
            &vec![&setup.env, 5u128, 4u128, 6u128],
        ),
    });
    setup
        .contract
        .vote(&setup.judge1, &dispute.dispute_id, &vote_);

    // Get initial balance and reputation
    let voter_before = setup.contract.get_user(&setup.judge1);
    assert_eq!(voter_before.balance, 0);
    assert_eq!(voter_before.reputation, 0);

    // Execute dispute (advance time past voting period)
    setup.env.ledger().set_timestamp(voting_ends_at + 1);

    let vote_result = setup.contract.execute(
        &setup.creator,
        &dispute.dispute_id,
        &Some(vec![&setup.env, 9u128, 3u128, 3u128]),
        &Some(vec![&setup.env, 15u128, 12u128, 18u128]),
    );

    assert_eq!(vote_result, DisputeStatus::CREATOR);

    // Claim reward
    setup.contract.claim_reward(&setup.judge1, &dispute.dispute_id);

    // Check updated balance and reputation
    let voter_after = setup.contract.get_user(&setup.judge1);
    assert_eq!(voter_after.balance, 10);
    assert_eq!(voter_after.reputation, 1);
}

#[test]
#[should_panic(expected = "Error(Contract, #39)")]
fn test_claim_reward_twice_fails() {
    let setup = create_test_data();

    let public_key = String::from_str(&setup.env, "public key random");
    setup
        .contract
        .anonymous_voting_setup(&setup.contract_admin, &setup.project_id, &public_key);

    let voting_ends_at = setup.env.ledger().timestamp() + 3600 * 24 * 2;
    let dispute = init_contract(&setup);

    // Judge1 votes
    let vote_ = VoteAnon::AnonymousVote(AnonymousVote {
        address: setup.judge1.clone(),
        weight: 3,
        encrypted_seeds: vec![
            &setup.env,
            String::from_str(&setup.env, "fafdas"),
            String::from_str(&setup.env, "fafdas"),
            String::from_str(&setup.env, "fafdas"),
        ],
        encrypted_votes: vec![
            &setup.env,
            String::from_str(&setup.env, "fafdas"),
            String::from_str(&setup.env, "fafdas"),
            String::from_str(&setup.env, "rewrewr"),
        ],
        commitments: setup.contract.build_commitments_from_votes(
            &dispute.dispute_id,
            &vec![&setup.env, 3u128, 1u128, 1u128],
            &vec![&setup.env, 5u128, 4u128, 6u128],
        ),
    });
    setup
        .contract
        .vote(&setup.judge1, &dispute.dispute_id, &vote_);

    // Execute dispute
    setup.env.ledger().set_timestamp(voting_ends_at + 1);
    setup.contract.execute(
        &setup.creator,
        &dispute.dispute_id,
        &Some(vec![&setup.env, 9u128, 3u128, 3u128]),
        &Some(vec![&setup.env, 15u128, 12u128, 18u128]),
    );

    // Claim reward first time - should succeed
    setup.contract.claim_reward(&setup.judge1, &dispute.dispute_id);

    // Try to claim again - should panic with AlreadyClaimed error (#39)
    setup.contract.claim_reward(&setup.judge1, &dispute.dispute_id);
}

#[test]
#[should_panic(expected = "Error(Contract, #37)")]
fn test_claim_reward_before_execution_fails() {
    let setup = create_test_data();

    let public_key = String::from_str(&setup.env, "public key random");
    setup
        .contract
        .anonymous_voting_setup(&setup.contract_admin, &setup.project_id, &public_key);

    let dispute = init_contract(&setup);

    // Judge1 votes
    let vote_ = VoteAnon::AnonymousVote(AnonymousVote {
        address: setup.judge1.clone(),
        weight: 3,
        encrypted_seeds: vec![
            &setup.env,
            String::from_str(&setup.env, "fafdas"),
            String::from_str(&setup.env, "fafdas"),
            String::from_str(&setup.env, "fafdas"),
        ],
        encrypted_votes: vec![
            &setup.env,
            String::from_str(&setup.env, "fafdas"),
            String::from_str(&setup.env, "fafdas"),
            String::from_str(&setup.env, "rewrewr"),
        ],
        commitments: setup.contract.build_commitments_from_votes(
            &dispute.dispute_id,
            &vec![&setup.env, 3u128, 1u128, 1u128],
            &vec![&setup.env, 5u128, 4u128, 6u128],
        ),
    });
    setup
        .contract
        .vote(&setup.judge1, &dispute.dispute_id, &vote_);

    // Try to claim reward before execution - should panic with ProposalActive error (#37)
    setup.contract.claim_reward(&setup.judge1, &dispute.dispute_id);
}
