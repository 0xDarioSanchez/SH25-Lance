use soroban_sdk::{Bytes, String, testutils::Address as _};

use crate::storage::dispute_status::DisputeStatus;
use crate::tests::test_utils::{create_test_data, compute_commit_hash};

#[test]
fn test_full_dispute_lifecycle() {
    let setup = create_test_data();

    // Step 1: Create voters
    setup.contract.new_voter(&setup.judge1);
    setup.contract.new_voter(&setup.judge2);
    setup.contract.new_voter(&setup.judge3);

    // Step 2: Create dispute (no longer requires public_key parameter)
    let proof = String::from_str(&setup.env, "Evidence IPFS hash");
    let dispute = setup.contract.create_dispute(
        &setup.project_id,
        &setup.creator,
        &setup.counterpart,
        &proof,
        &setup.voting_ends_at,
        &setup.contract_id,
    );

    assert_eq!(dispute.dispute_status, DisputeStatus::OPEN);

    // Step 3: Judges register to vote
    setup
        .contract
        .register_to_vote(&setup.judge1, &dispute.dispute_id);
    setup
        .contract
        .register_to_vote(&setup.judge2, &dispute.dispute_id);
    setup
        .contract
        .register_to_vote(&setup.judge3, &dispute.dispute_id);

    // Step 4: Judges commit their votes
    let secret1 = Bytes::from_slice(&setup.env, b"secret_judge_1");
    let secret2 = Bytes::from_slice(&setup.env, b"secret_judge_2");
    let secret3 = Bytes::from_slice(&setup.env, b"secret_judge_3");

    let commit_hash1 = compute_commit_hash(&setup.env, true, &secret1);
    let commit_hash2 = compute_commit_hash(&setup.env, true, &secret2);
    let commit_hash3 = compute_commit_hash(&setup.env, false, &secret3);

    setup
        .contract
        .commit_vote(&setup.judge1, &dispute.dispute_id, &commit_hash1);
    setup
        .contract
        .commit_vote(&setup.judge2, &dispute.dispute_id, &commit_hash2);
    setup
        .contract
        .commit_vote(&setup.judge3, &dispute.dispute_id, &commit_hash3);

    // Step 5: Creator reveals votes
    let votes = soroban_sdk::vec![&setup.env, true, true, false];
    let secrets = soroban_sdk::vec![&setup.env, secret1, secret2, secret3];

    let final_dispute =
        setup
            .contract
            .reveal_votes(&setup.creator, &dispute.dispute_id, &votes, &secrets);

    // Step 6: Verify final state
    assert_eq!(final_dispute.dispute_status, DisputeStatus::FINISHED);
    assert_eq!(final_dispute.votes_for, 2);
    assert_eq!(final_dispute.votes_against, 1);
    assert_eq!(final_dispute.winner, Some(setup.creator.clone()));
    assert!(final_dispute.finish_timestamp.is_some());
}

#[test]
fn test_multiple_disputes_independent() {
    let setup = create_test_data();

    // Create first dispute
    let proof1 = String::from_str(&setup.env, "Proof 1");
    let dispute1 = setup.contract.create_dispute(
        &setup.project_id,
        &setup.creator,
        &setup.counterpart,
        &proof1,
        &setup.voting_ends_at,
        &setup.contract_id,
    );

    // Create second dispute with different parties
    let creator2 = soroban_sdk::Address::generate(&setup.env);
    let counterpart2 = soroban_sdk::Address::generate(&setup.env);
    let proof2 = String::from_str(&setup.env, "Proof 2");

    let dispute2 = setup.contract.create_dispute(
        &setup.project_id,
        &creator2,
        &counterpart2,
        &proof2,
        &setup.voting_ends_at,
        &setup.contract_id,
    );

    // Register same judges for both disputes
    setup.contract.new_voter(&setup.judge1);
    setup
        .contract
        .register_to_vote(&setup.judge1, &dispute1.dispute_id);
    setup
        .contract
        .register_to_vote(&setup.judge1, &dispute2.dispute_id);

    // Commit different votes for each dispute
    let secret1 = Bytes::from_slice(&setup.env, b"secret_dispute1");
    let secret2 = Bytes::from_slice(&setup.env, b"secret_dispute2");

    let commit_hash1 = compute_commit_hash(&setup.env, true, &secret1);
    let commit_hash2 = compute_commit_hash(&setup.env, false, &secret2);

    setup
        .contract
        .commit_vote(&setup.judge1, &dispute1.dispute_id, &commit_hash1);
    setup
        .contract
        .commit_vote(&setup.judge1, &dispute2.dispute_id, &commit_hash2);

    // Verify disputes are independent
    assert_eq!(dispute1.dispute_id, 1);
    assert_eq!(dispute2.dispute_id, 2);
    assert_ne!(dispute1.creator, dispute2.creator);
}

#[test]
fn test_unanimous_vote_for_creator() {
    let setup = create_test_data();

    // Setup
    setup.contract.new_voter(&setup.judge1);
    setup.contract.new_voter(&setup.judge2);
    setup.contract.new_voter(&setup.judge3);

    let proof = String::from_str(&setup.env, "Strong evidence");
    let dispute = setup.contract.create_dispute(
        &setup.project_id,
        &setup.creator,
        &setup.counterpart,
        &proof,
        &setup.voting_ends_at,
        &setup.contract_id,
    );

    // All judges register
    setup
        .contract
        .register_to_vote(&setup.judge1, &dispute.dispute_id);
    setup
        .contract
        .register_to_vote(&setup.judge2, &dispute.dispute_id);
    setup
        .contract
        .register_to_vote(&setup.judge3, &dispute.dispute_id);

    // All judges vote FOR (true)
    let secret1 = Bytes::from_slice(&setup.env, b"s1");
    let secret2 = Bytes::from_slice(&setup.env, b"s2");
    let secret3 = Bytes::from_slice(&setup.env, b"s3");

    let commit_hash1 = compute_commit_hash(&setup.env, true, &secret1);
    let commit_hash2 = compute_commit_hash(&setup.env, true, &secret2);
    let commit_hash3 = compute_commit_hash(&setup.env, true, &secret3);

    setup
        .contract
        .commit_vote(&setup.judge1, &dispute.dispute_id, &commit_hash1);
    setup
        .contract
        .commit_vote(&setup.judge2, &dispute.dispute_id, &commit_hash2);
    setup
        .contract
        .commit_vote(&setup.judge3, &dispute.dispute_id, &commit_hash3);

    // Reveal
    let votes = soroban_sdk::vec![&setup.env, true, true, true];
    let secrets = soroban_sdk::vec![&setup.env, secret1, secret2, secret3];

    let final_dispute =
        setup
            .contract
            .reveal_votes(&setup.creator, &dispute.dispute_id, &votes, &secrets);

    assert_eq!(final_dispute.votes_for, 3);
    assert_eq!(final_dispute.votes_against, 0);
    assert_eq!(final_dispute.winner, Some(setup.creator.clone()));
}

#[test]
fn test_unanimous_vote_for_counterpart() {
    let setup = create_test_data();

    // Setup
    setup.contract.new_voter(&setup.judge1);
    setup.contract.new_voter(&setup.judge2);
    setup.contract.new_voter(&setup.judge3);

    let proof = String::from_str(&setup.env, "Weak evidence");
    let dispute = setup.contract.create_dispute(
        &setup.project_id,
        &setup.creator,
        &setup.counterpart,
        &proof,
        &setup.voting_ends_at,
        &setup.contract_id,
    );

    // All judges register
    setup
        .contract
        .register_to_vote(&setup.judge1, &dispute.dispute_id);
    setup
        .contract
        .register_to_vote(&setup.judge2, &dispute.dispute_id);
    setup
        .contract
        .register_to_vote(&setup.judge3, &dispute.dispute_id);

    // All judges vote AGAINST (false)
    let secret1 = Bytes::from_slice(&setup.env, b"s1");
    let secret2 = Bytes::from_slice(&setup.env, b"s2");
    let secret3 = Bytes::from_slice(&setup.env, b"s3");

    let commit_hash1 = compute_commit_hash(&setup.env, false, &secret1);
    let commit_hash2 = compute_commit_hash(&setup.env, false, &secret2);
    let commit_hash3 = compute_commit_hash(&setup.env, false, &secret3);

    setup
        .contract
        .commit_vote(&setup.judge1, &dispute.dispute_id, &commit_hash1);
    setup
        .contract
        .commit_vote(&setup.judge2, &dispute.dispute_id, &commit_hash2);
    setup
        .contract
        .commit_vote(&setup.judge3, &dispute.dispute_id, &commit_hash3);

    // Reveal
    let votes = soroban_sdk::vec![&setup.env, false, false, false];
    let secrets = soroban_sdk::vec![&setup.env, secret1, secret2, secret3];

    let final_dispute =
        setup
            .contract
            .reveal_votes(&setup.creator, &dispute.dispute_id, &votes, &secrets);

    assert_eq!(final_dispute.votes_for, 0);
    assert_eq!(final_dispute.votes_against, 3);
    assert_eq!(final_dispute.winner, Some(setup.counterpart.clone()));
}

#[test]
fn test_single_judge_decides() {
    let setup = create_test_data();

    // Only one judge
    setup.contract.new_voter(&setup.judge1);

    let proof = String::from_str(&setup.env, "Evidence");
    let dispute = setup.contract.create_dispute(
        &setup.project_id,
        &setup.creator,
        &setup.counterpart,
        &proof,
        &setup.voting_ends_at,
        &setup.contract_id,
    );

    setup
        .contract
        .register_to_vote(&setup.judge1, &dispute.dispute_id);

    let secret = Bytes::from_slice(&setup.env, b"solo_secret");
    let commit_hash = compute_commit_hash(&setup.env, true, &secret);
    setup
        .contract
        .commit_vote(&setup.judge1, &dispute.dispute_id, &commit_hash);

    let votes = soroban_sdk::vec![&setup.env, true];
    let secrets = soroban_sdk::vec![&setup.env, secret];

    let final_dispute =
        setup
            .contract
            .reveal_votes(&setup.creator, &dispute.dispute_id, &votes, &secrets);

    assert_eq!(final_dispute.votes_for, 1);
    assert_eq!(final_dispute.votes_against, 0);
    assert_eq!(final_dispute.winner, Some(setup.creator.clone()));
}

#[test]
fn test_large_number_of_judges() {
    let setup = create_test_data();

    let proof = String::from_str(&setup.env, "Evidence");
    let dispute = setup.contract.create_dispute(
        &setup.project_id,
        &setup.creator,
        &setup.counterpart,
        &proof,
        &setup.voting_ends_at,
        &setup.contract_id,
    );

    // Create 5 judges
    let mut judges = soroban_sdk::vec![&setup.env];
    for _ in 0..5 {
        judges.push_back(soroban_sdk::Address::generate(&setup.env));
    }

    let mut secrets = soroban_sdk::vec![&setup.env];
    let mut votes = soroban_sdk::vec![&setup.env];

    // Register and commit votes (3 for, 2 against)
    for i in 0..judges.len() {
        let judge = judges.get(i).unwrap();
        setup.contract.new_voter(&judge);
        setup.contract.register_to_vote(&judge, &dispute.dispute_id);

        let vote = i < 3; // First 3 vote true, last 2 vote false
        let secret_bytes = if i == 0 {
            b"secret_0"
        } else if i == 1 {
            b"secret_1"
        } else if i == 2 {
            b"secret_2"
        } else if i == 3 {
            b"secret_3"
        } else {
            b"secret_4"
        };
        let secret = Bytes::from_slice(&setup.env, secret_bytes);

        let commit_hash = compute_commit_hash(&setup.env, vote, &secret);
        setup
            .contract
            .commit_vote(&judge, &dispute.dispute_id, &commit_hash);

        votes.push_back(vote);
        secrets.push_back(secret);
    }

    // Reveal all votes
    let final_dispute =
        setup
            .contract
            .reveal_votes(&setup.creator, &dispute.dispute_id, &votes, &secrets);

    assert_eq!(final_dispute.votes_for, 3);
    assert_eq!(final_dispute.votes_against, 2);
    assert_eq!(final_dispute.winner, Some(setup.creator.clone()));
}
