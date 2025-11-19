use soroban_sdk::{token::{self}, Address, Env};

use crate::storage::{error::Error, storage::DataKey};

pub(crate) fn set_token(env: &Env, token: &Address) {
    let key = DataKey::Token;

    env.storage().instance().set(&key, token);
}

pub(crate) fn get_token(env: &Env) -> Result<Address, Error> {
    let key = DataKey::Token;

    env.storage()
        .instance()
        .get(&key)
        .ok_or(Error::ContractNotInitialized)
}

pub fn token_transfer(env: &Env, from: &Address, to: &Address, amount: &i128) -> Result<(), Error> {
    let token_id = get_token(env)?;
    let token = token::Client::new(env, &token_id);
    token.transfer(from, to, amount);
    Ok(())
}