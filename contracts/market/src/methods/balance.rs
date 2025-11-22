use soroban_sdk::{Env, Address};
use crate::storage::storage::DataKey;

pub fn set_balance(env: &Env, user: &Address, amount: i128) {
    let key = DataKey::Balances(user.clone());
    env.storage().persistent().set(&key, &amount);
}

pub fn get_balance(env: &Env, user: &Address) -> i128 {
    let key = DataKey::Balances(user.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}