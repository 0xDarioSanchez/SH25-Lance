#![cfg(test)]

use crate::tests::config::{constants::BASE_MINT_AMOUNT, contract::ContractTest};

#[test]
fn test_approve_milestone() {
    let ContractTest {
        env,
        contract,
        employee_1,
        employer_1,
        token,
        ..
    } = ContractTest::setup();

    env.mock_all_auths();
    let (token_client, _, _) = token;

    let service_id_1: u32 = 1;
    let one_day_duration: u64 = 1; // days
    let milestone_payment: i128 = 1000;

    contract.create_service(
        &employee_1,
        &employer_1,
        &service_id_1,
        &one_day_duration,
        &None,
        &milestone_payment,
    );

    contract.accept_service(&employer_1, &service_id_1);

    assert_eq!(token_client.balance(&contract.address), milestone_payment);
    assert_eq!(token_client.balance(&employee_1), BASE_MINT_AMOUNT);
    assert_eq!(
        token_client.balance(&employer_1),
        BASE_MINT_AMOUNT - milestone_payment
    );
    assert_eq!(contract.get_balance(&employee_1), 0);
    assert_eq!(contract.get_balance(&employer_1), 0);

    contract.approve_milestone(&employer_1, &service_id_1);

    assert_eq!(token_client.balance(&contract.address), milestone_payment);
    assert_eq!(token_client.balance(&employee_1), BASE_MINT_AMOUNT);
    assert_eq!(
        token_client.balance(&employer_1),
        BASE_MINT_AMOUNT - milestone_payment
    );
    assert_eq!(contract.get_balance(&employer_1), 0);
    assert_eq!(contract.get_balance(&employee_1), milestone_payment);
}
