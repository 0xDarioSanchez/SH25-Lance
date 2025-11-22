use soroban_sdk::{contracttype, Address, Env, String, Vec};
use crate::storage::{error::Error, storage::DataKey};

#[derive(Clone)]
#[contracttype]
pub struct User {
    pub address: Address,
    pub is_employee: bool,
    pub is_employer: bool,
    pub is_judge: bool,
    pub personal_data: Option<String>,
    pub delegates: Vec<Address>,
}

pub(crate) fn get_user(env: &Env, user: Address) -> Result<User, Error> {
    let key = DataKey::Users(user);

    env.storage()
        .instance()
        .get(&key)
        .ok_or(Error::UserNotFound)
}

pub(crate) fn set_user(
        env: &Env,
        user: Address,
        is_employee: bool,
        is_employer: bool,
        is_judge: bool,
        personal_data: Option<String>,
    ) {
    let new_user = User {
        address: user.clone(),
        is_employee,
        is_employer,
        is_judge,
        personal_data,
        delegates: Vec::new(env),
    };

    let key = DataKey::Users(user.clone());

    env.storage().instance().set(&key, &new_user);
}

pub(crate) fn vote_delegate(
        env: &Env,
        judge: Address,
        delegator: Address,
    ) 
    -> Result<(), Error> {

    let mut user = get_user(env, judge.clone())?;
    // TODO: Check if already exists
    user.delegates.push_back(delegator);

    let key = DataKey::Users(judge);

    env.storage().instance().set(&key, &user);

    Ok(())
}
