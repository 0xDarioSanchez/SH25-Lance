use soroban_sdk::{String, testutils::Address as _};

use crate::storage::dispute_status::DisputeStatus;
use crate::tests::test_utils::{create_test_data, init_contract};

#[test]
fn test_create_dispute_success() {
    let setup = create_test_data();

    let proof = String::from_str(&setup.env, "IPFS_HASH_PROOF_1");

    let dispute = setup.contract.create_dispute(
        &setup.project_id,
        &setup.creator,
        &setup.counterpart,
        &proof,
        &setup.voting_ends_at,
        &setup.contract_id,
    );

    assert_eq!(dispute.dispute_id, 1);
    assert_eq!(dispute.creator, setup.creator);
    assert_eq!(dispute.counterpart, setup.counterpart);
    assert_eq!(dispute.creator_proves, proof);
    assert_eq!(dispute.dispute_status, DisputeStatus::OPEN);
    assert_eq!(dispute.votes_for, 0);
    assert_eq!(dispute.votes_against, 0);
    assert_eq!(dispute.called_contract, setup.contract_id);
    assert!(dispute.winner.is_none());
    assert!(dispute.finish_timestamp.is_none());
}

#[test]
fn test_create_multiple_disputes() {
    let setup = create_test_data();

    let proof1 = String::from_str(&setup.env, "PROOF_1");
    let proof2 = String::from_str(&setup.env, "PROOF_2");

    let dispute1 = setup.contract.create_dispute(
        &setup.project_id,
        &setup.creator,
        &setup.counterpart,
        &proof1,
        &setup.voting_ends_at,
        &setup.contract_id,
    );

    let new_creator = soroban_sdk::Address::generate(&setup.env);
    let new_counterpart = soroban_sdk::Address::generate(&setup.env);

    let dispute2 = setup.contract.create_dispute(
        &setup.project_id,
        &new_creator,
        &new_counterpart,
        &proof2,
        &setup.voting_ends_at,
        &setup.contract_id,
    );

    assert_eq!(dispute1.dispute_id, 1);
    assert_eq!(dispute2.dispute_id, 2);
    assert_ne!(dispute1.creator, dispute2.creator);
}

#[test]
fn test_dispute_initial_state() {
    let setup = create_test_data();
    let dispute = init_contract(&setup);

    // Verify initial dispute state
    assert_eq!(dispute.dispute_status, DisputeStatus::OPEN);
    // init_contract already calls register_to_vote for judge1, but that doesn't increment able_to_vote
    // Check that we have at least 0 voters initially (before registration)
    assert_eq!(dispute.voters.len(), 0); // No commits yet
    assert_eq!(dispute.vote_commits.len(), 0);
    assert_eq!(dispute.votes.len(), 0);
    assert!(!dispute.waiting_for_judges);
}

#[test]
fn test_dispute_timestamps() {
    let setup = create_test_data();
    let proof = String::from_str(&setup.env, "PROOF");

    let initial_time = setup.env.ledger().timestamp();

    let dispute = setup.contract.create_dispute(
        &setup.project_id,
        &setup.creator,
        &setup.counterpart,
        &proof,
        &setup.voting_ends_at,
        &setup.contract_id,
    );

    assert_eq!(dispute.initial_timestamp, initial_time);
    assert!(dispute.finish_timestamp.is_none());
}
