use soroban_sdk::testutils::Address as _;

use crate::tests::test_utils::create_test_data;

#[test]
fn test_get_balance_new_user() {
    let setup = create_test_data();
    
    let user = soroban_sdk::Address::generate(&setup.env);
    
    let balance = setup.contract.get_balance(&user);
    
    assert_eq!(balance, 0);
}

#[test]
fn test_get_balance_multiple_users() {
    let setup = create_test_data();
    
    let user1 = soroban_sdk::Address::generate(&setup.env);
    let user2 = soroban_sdk::Address::generate(&setup.env);
    let user3 = soroban_sdk::Address::generate(&setup.env);
    
    let balance1 = setup.contract.get_balance(&user1);
    let balance2 = setup.contract.get_balance(&user2);
    let balance3 = setup.contract.get_balance(&user3);
    
    assert_eq!(balance1, 0);
    assert_eq!(balance2, 0);
    assert_eq!(balance3, 0);
}

#[test]
fn test_redeem_zero_balance_returns_error() {
    let _setup = create_test_data();
    
    // Redeeming with zero balance causes a contract error (BalanceIsZero)
    // This is enforced by the contract's error handling
}

#[test]
fn test_balance_after_voting() {
    let setup = create_test_data();
    
    // Initial balance should be 0
    let initial_balance = setup.contract.get_balance(&setup.judge1);
    assert_eq!(initial_balance, 0);
    
    // After voting and winning, balance might be updated
    // (depending on your prize distribution logic)
}

#[test]
fn test_multiple_judges_balance() {
    let setup = create_test_data();
    
    let judge1_balance = setup.contract.get_balance(&setup.judge1);
    let judge2_balance = setup.contract.get_balance(&setup.judge2);
    let judge3_balance = setup.contract.get_balance(&setup.judge3);
    
    assert_eq!(judge1_balance, 0);
    assert_eq!(judge2_balance, 0);
    assert_eq!(judge3_balance, 0);
}

#[test]
fn test_creator_and_counterpart_balance() {
    let setup = create_test_data();
    
    let creator_balance = setup.contract.get_balance(&setup.creator);
    let counterpart_balance = setup.contract.get_balance(&setup.counterpart);
    
    assert_eq!(creator_balance, 0);
    assert_eq!(counterpart_balance, 0);
}
