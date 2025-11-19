use soroban_sdk::{Env, Address};
use crate::storage::{storage::DataKey, error::Error};
use crate::methods::token::token_transfer;

pub fn set_balance(env: &Env, user: &Address, amount: i128) {
    let key = DataKey::Balances(user.clone());
    env.storage().persistent().set(&key, &amount);
}

pub fn get_balance(env: &Env, user: &Address) -> i128 {
    let key = DataKey::Balances(user.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn redeem(env: &Env, employee: Address) -> Result<i128, Error> {
    employee.require_auth();

    let balance = get_balance(env, &employee);

    if balance == 0 {
        return Err(Error::BalanceIsZero);
    }

    set_balance(env, &employee, 0);

    token_transfer(env, &env.current_contract_address(), &employee, &balance)?;

    Ok(balance)
}