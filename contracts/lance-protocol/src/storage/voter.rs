use soroban_sdk::{contracttype, Address, Env, Vec};
use crate::storage::{error::Error, storage::DataKey};

#[derive(Clone)]
#[contracttype]
pub struct Voter {
    pub address: Address,
    pub delegates: Vec<Address>,
    pub reputation: u32,
    pub balance: i128,
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
        reputation: 0,
        balance: 0,
    };

    let key = DataKey::Voters(address.clone());

    env.storage().instance().set(&key, &new_voter);
}

pub(crate) fn update_voter(
    env: &Env,
    mut voter: Voter,
    balance_delta: i128,
    reputation_delta: u32,
) {
    voter.balance += balance_delta;
    voter.reputation += reputation_delta;

    let key = DataKey::Voters(voter.address.clone());
    env.storage().instance().set(&key, &voter);
}
