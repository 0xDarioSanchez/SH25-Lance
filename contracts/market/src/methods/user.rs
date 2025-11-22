use soroban_sdk::{Address, Env, String, Vec};

use crate::storage::{error::Error, storage::DataKey, user::User};

pub fn add_user(
    env: Env,
    user: Address,
    is_employee: bool,
    is_employer: bool,
    is_judge: bool,
    personal_data: Option<String>,
) -> Result<(), Error> {
    user.require_auth();

    let new_user = User {
        address: user.clone(),
        is_employee,
        is_employer,
        is_judge,
        personal_data,
        delegates: Vec::new(&env),
        //banned: false,   //TODO implement banned
    };

    let key = DataKey::Users(user);

    env.storage().instance().set(&key, &new_user);

    Ok(())
}
