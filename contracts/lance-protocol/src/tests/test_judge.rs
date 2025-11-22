use soroban_sdk::{
    IntoVal, Map, String, Symbol, Val,
    testutils::{Events, Ledger},
    vec,
};

use crate::{
    storage::{
        dispute_status::DisputeStatus,
        vote::{AnonymousVote, Badge, VoteAnon},
    },
    tests::test_utils::{create_test_data, init_contract},
};

#[test]
fn test_vote_maths() {
    let setup = create_test_data();

    let public_key = String::from_str(&setup.env, "public key random");
    setup
        .contract
        .anonymous_voting_setup(&setup.contract_admin, &setup.project_id, &public_key);

    let voting_ends_at = setup.env.ledger().timestamp() + 3600 * 24 * 2;
    let dispute = init_contract(&setup);

    /*
       let proposal_id = setup.contract.create_proposal(
           &setup.grogu,
           &id,
           &title,
           &ipfs,
           &voting_ends_at,
           &false,
           &None,
       );
       assert_eq!(proposal_id, 0);
    */

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

    setup.env.ledger().set_timestamp(voting_ends_at + 1);

    let vote_result = setup.contract.execute(
        &setup.creator,
        &dispute.dispute_id,
        &Some(vec![&setup.env, 9u128, 3u128, 3u128]),
        &Some(vec![&setup.env, 15u128, 12u128, 18u128]),
    );

    assert_eq!(vote_result, DisputeStatus::CREATOR);

    /*
    // test build_commitments_from_votes and abstain
    let abstain_vote = VoteAnon::AnonymousVote(AnonymousVote {
        address: setup.creator.clone(),
        weight: Badge::Verified as u32,
        encrypted_seeds: vec![
            &setup.env,
            String::from_str(&setup.env, "0"),
            String::from_str(&setup.env, "0"),
            String::from_str(&setup.env, "0"),
        ],
        encrypted_votes: vec![
            &setup.env,
            String::from_str(&setup.env, "0"),
            String::from_str(&setup.env, "0"),
            String::from_str(&setup.env, "1"),
        ],
        commitments: setup.contract.build_commitments_from_votes(
            &dispute.dispute_id,
            &vec![&setup.env, 0u128, 0u128, 1u128],
            &vec![&setup.env, 0u128, 0u128, 0u128],
        ),
    });

    assert_eq!(
        dispute.vote_data.votes,
        vec![&setup.env, abstain_vote.clone()]
    );*/
}
