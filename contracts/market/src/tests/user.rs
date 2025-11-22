#![cfg(test)]

use crate::tests::config::contract::ContractTest;
use soroban_sdk::testutils::Address as TestAddress;
use soroban_sdk::{Address, Env, String};
// use soroban_sdk::testutils::Env as TestEnv; // if you need mock_all_auths

#[test]
fn test_create_employee_user() {
    let mut contract_test = ContractTest::setup();
    let env = &mut contract_test.env;
    let contract = &contract_test.contract;
    let employee_1 = contract_test.employee_1;

    env.mock_all_auths();

    contract.new_user(&employee_1, &true, &false, &false, &None);

    let user_data = contract.get_user(&employee_1);

    assert_eq!(user_data.address, employee_1);
    assert!(user_data.is_employee);
    assert!(!user_data.is_employer);
    assert!(!user_data.is_judge);
    assert!(user_data.personal_data.is_none());
}

#[test]
fn test_create_employer_user() {
    let ContractTest {
        env,
        contract,
        employer_1,
        ..
    } = ContractTest::setup();

    env.mock_all_auths();

    contract.new_user(&employer_1, &false, &true, &false, &None);

    let user_data = contract.get_user(&employer_1);

    assert_eq!(user_data.address, employer_1);
    assert!(!user_data.is_employee);
    assert!(user_data.is_employer);
    assert!(!user_data.is_judge);
    assert!(user_data.personal_data.is_none());
}

#[test]
fn test_create_user_with_personal_data() {
    let ContractTest {
        env,
        contract,
        employee_1,
        ..
    } = ContractTest::setup();

    env.mock_all_auths();

    let personal_data = String::from_str(&env, "Employee 1");

    contract.new_user(
        &employee_1,
        &true,
        &false,
        &false,
        &Some(personal_data.clone()),
    );

    let user_data = contract.get_user(&employee_1);
    assert_eq!(user_data.personal_data, Some(personal_data));
}

#[test]
fn test_multiple_users_are_independent() {
    let ContractTest {
        env,
        contract,
        employee_1,
        employer_1,
        ..
    } = ContractTest::setup();

    env.mock_all_auths();

    contract.new_user(&employee_1, &true, &false, &false, &None);
    contract.new_user(&employer_1, &false, &true, &false, &None);

    let data1 = contract.get_user(&employee_1);
    let data2 = contract.get_user(&employer_1);

    assert_ne!(data1.address, data2.address);
    assert!(data1.is_employee && !data2.is_employee);
    assert!(data2.is_employer && !data1.is_employer);
}

#[test]
fn test_add_and_get_user() {
    let mut contract_test = ContractTest::setup();
    let env = &mut contract_test.env;
    let contract = &contract_test.contract;
    let employee_1 = contract_test.employee_1;

    env.mock_all_auths();

    let personal_data = String::from_str(env, "Employee 1");
    contract.new_user(
        &employee_1,
        &true,  // is_employee
        &false, // is_employer
        &false, // is_judge
        &Some(personal_data.clone()),
    );

    let user_data = contract.get_user(&employee_1);

    assert_eq!(user_data.address, employee_1);
    assert_eq!(user_data.is_employee, true);
    assert_eq!(user_data.is_employer, false);
    assert_eq!(user_data.is_judge, false);
    assert_eq!(user_data.personal_data, Some(personal_data));
}
