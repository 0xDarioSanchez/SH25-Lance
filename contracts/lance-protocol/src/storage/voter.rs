use soroban_sdk::{contracttype, Address, Env, Vec};
use crate::storage::{error::Error, storage::DataKey};

#[derive(Clone)]
#[contracttype]
pub struct Voter {
    pub address: Address,
    pub delegates: Vec<Address>,
}

pub(crate) fn get_voter(env: &Env, voter: Address) -> Result<Voter, Error> {
    let key = DataKey::Voters(voter);

    env.storage()
        .instance()
        .get(&key)
        .ok_or(Error::UserNotFound)
}

pub(crate) fn set_voter(
        env: &Env,
        address: Address,
    ) {
    let new_voter = Voter {
        address: address.clone(),
        delegates: Vec::new(env),
    };

    let key = DataKey::Voters(address.clone());

    env.storage().instance().set(&key, &new_voter);
}
