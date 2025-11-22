use soroban_sdk::{Address, Env};

use super::{
    admin::{has_admin, set_admin},
    token::set_token,
};
use crate::storage::error::Error;

pub fn initialize(
    env: &Env,
    admin: Address,
    token: Address,
) -> Result<(), Error> {
    if has_admin(env) {
        return Err(Error::ContractInitialized);
    }

    set_admin(&env, &admin);
    set_token(&env, &token);

    // events::contract::contract_initialized(&env, &admin); //TODO

    Ok(())
}
