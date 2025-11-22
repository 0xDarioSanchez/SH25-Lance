#![cfg(test)]

use crate::tests::config::constants::BASE_MINT_AMOUNT;
use crate::tests::config::contract::ContractTest;

#[test]
fn test_redeem_after_approval() {
    let ContractTest {
        env,
        contract,
        employee_1,
        employer_1,
        token,
        service_1: (service_id_1, one_day_duration, milestone_payment),
        ..
    } = ContractTest::setup();

    // *****************
    // ***** Given *****
    // *****************
    env.mock_all_auths();
    let (token_client, _, _) = token;

    contract.create_service(
        &employee_1,
        &employer_1,
        &service_id_1,
        &one_day_duration,
        &None,
        &milestone_payment,
    );

    contract.accept_service(&employer_1, &service_id_1);

    contract.approve_service(&employer_1, &service_id_1);

    assert_eq!(token_client.balance(&contract.address), milestone_payment);
    assert_eq!(token_client.balance(&employee_1), BASE_MINT_AMOUNT);
    assert_eq!(
        token_client.balance(&employer_1),
        BASE_MINT_AMOUNT - milestone_payment
    );
    assert_eq!(contract.get_balance(&employee_1), milestone_payment);

    // *****************
    //  ***** When *****
    // *****************

    contract.redeem(&employee_1);

    // ****************
    // ***** Then *****
    // ****************
    assert_eq!(contract.get_balance(&employee_1), 0);

    assert_eq!(token_client.balance(&contract.address), 0);
    assert_eq!(
        token_client.balance(&employee_1),
        BASE_MINT_AMOUNT + milestone_payment
    );
}
