#![cfg(test)]

use soroban_sdk::String;

use crate::tests::config::constants::BASE_MINT_AMOUNT;
use crate::{storage::service_status::ServiceStatus, tests::config::contract::ContractTest};

#[test]
fn test_add_and_new_service() {
    let ContractTest {
        env,
        contract,
        employee_1,
        employer_1,
        ..
    } = ContractTest::setup();

    // *****************
    // ***** Given *****
    // *****************
    env.mock_all_auths();

    let service_id_1: u32 = 1;
    let one_day_duration: u64 = 1; // days
    let milestone_payment: i128 = 1000;

    // Now you can call other contract methods
    let service_metadata = String::from_str(&env, "Service 1");

    // *****************
    //  ***** When *****
    // *****************
    contract.create_service(
        &employee_1,
        &employer_1,
        &service_id_1,
        &one_day_duration,
        &Some(service_metadata.clone()),
        &milestone_payment,
    );

    // ****************
    // ***** Then *****
    // ****************
    let service_data = contract.get_service(&service_id_1);

    assert_eq!(service_data.id, service_id_1);
    assert_eq!(service_data.employee, employee_1);
    assert_eq!(service_data.employer, employer_1);
    assert_eq!(service_data.duration, one_day_duration * 86400); // in seconds
    assert_eq!(service_data.metadata, Some(service_metadata));
    assert_eq!(service_data.milestone_payment, milestone_payment);
    assert_eq!(service_data.status, ServiceStatus::CREATED);
}

#[test]
fn test_accept_service() {
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

    assert_eq!(token_client.balance(&contract.address), 0);
    assert_eq!(token_client.balance(&employee_1), BASE_MINT_AMOUNT);
    assert_eq!(token_client.balance(&employer_1), BASE_MINT_AMOUNT);

    contract.create_service(
        &employee_1,
        &employer_1,
        &service_id_1,
        &one_day_duration,
        &None,
        &milestone_payment,
    );

    // *****************
    //  ***** When *****
    // *****************
    contract.accept_service(&employer_1, &service_id_1);

    // ****************
    // ***** Then *****
    // ****************

    // Assert that the service status is now ACCEPTED
    let service_data = contract.get_service(&service_id_1);
    assert_eq!(service_data.status, ServiceStatus::ACCEPTED);

    // Assert balances after accepting the service
    assert_eq!(token_client.balance(&employee_1), BASE_MINT_AMOUNT);
    assert_eq!(
        token_client.balance(&employer_1),
        BASE_MINT_AMOUNT - milestone_payment
    );
    assert_eq!(token_client.balance(&contract.address), milestone_payment);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")] // Service is deleted after approval, so should panic with Err::ServiceNotFound
fn test_approve_service() {
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

    assert_eq!(token_client.balance(&contract.address), milestone_payment);
    assert_eq!(token_client.balance(&employee_1), BASE_MINT_AMOUNT);
    assert_eq!(
        token_client.balance(&employer_1),
        BASE_MINT_AMOUNT - milestone_payment
    );
    assert_eq!(contract.get_balance(&employee_1), 0);
    assert_eq!(contract.get_balance(&employer_1), 0);

    // *****************
    //  ***** When *****
    // *****************
    contract.approve_service(&employer_1, &service_id_1);

    // ****************
    // ***** Then *****
    // ****************

    // Assert that the service status is now WAITING
    let service_data = contract.get_service(&service_id_1);
    assert_eq!(service_data.status, ServiceStatus::WAITING);

    assert_eq!(token_client.balance(&contract.address), milestone_payment);
    assert_eq!(token_client.balance(&employee_1), BASE_MINT_AMOUNT);
    assert_eq!(
        token_client.balance(&employer_1),
        BASE_MINT_AMOUNT - milestone_payment
    );
    assert_eq!(contract.get_balance(&employer_1), 0);
    assert_eq!(contract.get_balance(&employee_1), milestone_payment);
}
