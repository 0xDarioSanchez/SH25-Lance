use soroban_sdk::{IntoVal, Map, String, Symbol, Val, testutils::Events, vec};

use crate::tests::test_utils::{create_test_data, init_contract};

#[test]
fn test_vote_maths() {
    let setup = create_test_data();
    let dispute = init_contract(&setup);

    let public_key = String::from_str(&setup.env, "public key random");
    setup
        .contract
        .anonymous_voting_setup(&setup.judge1, &dispute.dispute_id, &public_key);

    let all_events = setup.env.events().all();
    assert_eq!(
        all_events,
        vec![
            &setup.env,
            (
                setup.contract_id.clone(),
                (
                    Symbol::new(&setup.env, "anonymous_voting_setup"),
                    dispute.dispute_id.clone()
                )
                    .into_val(&setup.env),
                Map::<Symbol, Val>::from_array(
                    &setup.env,
                    [
                        (
                            Symbol::new(&setup.env, "judge"),
                            setup.judge1.clone().into_val(&setup.env)
                        ),
                        (
                            Symbol::new(&setup.env, "public_key"),
                            public_key.into_val(&setup.env)
                        ),
                    ],
                )
                .into_val(&setup.env),
            ),
        ]
    );

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
}
