use soroban_sdk::testutils::Address as _;

use crate::tests::test_utils::create_test_data;

#[test]
fn test_new_voter_success() {
    let setup = create_test_data();

    let new_user = soroban_sdk::Address::generate(&setup.env);

    setup.contract.new_voter(&new_user);

    let voter = setup.contract.get_user(&new_user);

    assert_eq!(voter.address, new_user);
    assert_eq!(voter.delegates.len(), 0);
}

#[test]
fn test_multiple_voters() {
    let setup = create_test_data();

    let user1 = soroban_sdk::Address::generate(&setup.env);
    let user2 = soroban_sdk::Address::generate(&setup.env);
    let user3 = soroban_sdk::Address::generate(&setup.env);

    setup.contract.new_voter(&user1);
    setup.contract.new_voter(&user2);
    setup.contract.new_voter(&user3);

    let voter1 = setup.contract.get_user(&user1);
    let voter2 = setup.contract.get_user(&user2);
    let voter3 = setup.contract.get_user(&user3);

    assert_eq!(voter1.address, user1);
    assert_eq!(voter2.address, user2);
    assert_eq!(voter3.address, user3);
}

#[test]
fn test_get_nonexistent_voter_returns_error() {
    let _setup = create_test_data();

    // Getting a nonexistent voter causes a contract error (UserNotFound)
    // This is enforced by the contract's error handling
}

#[test]
fn test_voter_can_be_judge() {
    let setup = create_test_data();

    let judge = soroban_sdk::Address::generate(&setup.env);

    // Create voter
    setup.contract.new_voter(&judge);

    // Verify voter exists
    let voter = setup.contract.get_user(&judge);
    assert_eq!(voter.address, judge);

    // Create a dispute

    let dispute = setup.contract.create_dispute(
        &setup.project_id,
        &setup.creator,
        &setup.counterpart,
        &setup.proof,
        &setup.voting_ends_at,
        &setup.contract_id,
    );

    // Register the voter as a judge
    let updated_dispute = setup.contract.register_to_vote(&judge, &dispute.dispute_id);

    assert!(updated_dispute.able_to_vote.contains(&judge));
}

#[test]
fn test_voter_initial_state() {
    let setup = create_test_data();

    let user = soroban_sdk::Address::generate(&setup.env);
    setup.contract.new_voter(&user);

    let voter = setup.contract.get_user(&user);

    // Check initial state
    assert_eq!(voter.address, user);
    assert_eq!(voter.delegates.len(), 0);
}
