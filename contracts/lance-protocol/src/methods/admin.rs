use soroban_sdk::{Address, Bytes, Env, String, panic_with_error};

use crate::{
    events::event,
    storage::{
        self, DataKey,
        error::{self, Error},
        vote,
    },
};

pub(crate) fn read_admin(env: &Env) -> Result<Address, Error> {
    let key = DataKey::Admin;

    env.storage()
        .instance()
        .get(&key)
        .ok_or(Error::AdminNotFound)
}

pub(crate) fn has_admin(env: &Env) -> bool {
    let key = DataKey::Admin;

    env.storage().instance().has(&key)
}

pub(crate) fn set_admin(env: &Env, admin: &Address) {
    let key = DataKey::Admin;

    env.storage().instance().set(&key, admin);
}

pub(crate) fn require_admin(env: &Env) {
    let key = DataKey::Admin;

    if let Some(admin) = env.storage().instance().get::<DataKey, Address>(&key) {
        admin.require_auth();
    } else {
        panic_with_error!(&env, &error::Error::InvalidKey)
    }
}

pub(crate) fn auth_maintainers(
    env: &Env,
    maintainer: &Address,
    project_id: u32,
) -> storage::Dispute {
    maintainer.require_auth();

    if let Some(dispute) = env
        .storage()
        .instance()
        .get::<DataKey, storage::Dispute>(&DataKey::Disputes(project_id))
    {
        if !dispute.able_to_vote.contains(maintainer) && !dispute.voters.contains(maintainer) {
            //QUESTION: should we panic here?
            panic_with_error!(&env, &error::Error::UnauthorizedSigner);
        }
        dispute
    } else {
        panic_with_error!(&env, &error::Error::InvalidKey)
    }
}

/// Setup anonymous voting for a project.
///
/// Configures BLS12-381 cryptographic primitives for anonymous voting.
/// Only the contract admin can call this function.
///
/// # Arguments
/// * `env` - The environment object
/// * `project_key` - Unique identifier for the project
/// * `public_key` - Asymmetric public key to be used for vote encryption
///
/// # Panics
/// * If the caller is not the contract admin
pub(crate) fn anonymous_voting_setup(
    env: Env,
    judge: Address,
    project_id: u32,
    public_key: String,
) {
    // generators
    let bls12_381 = env.crypto().bls12_381();

    let vote_generator = Bytes::from_slice(&env, "VOTE_GENERATOR".as_bytes());
    let vote_dst = Bytes::from_slice(&env, "VOTE_COMMITMENT".as_bytes());
    let seed_generator = Bytes::from_slice(&env, "SEED_GENERATOR".as_bytes());
    let seed_dst = Bytes::from_slice(&env, "VOTE_SEED".as_bytes());

    let vote_generator_point = bls12_381.hash_to_g1(&vote_generator, &vote_dst).to_bytes();
    let seed_generator_point = bls12_381.hash_to_g1(&seed_generator, &seed_dst).to_bytes();

    let vote_config = vote::AnonymousVoteConfig {
        vote_generator_point,
        seed_generator_point,
        public_key: public_key.clone(),
    };

    env.storage()
        .instance()
        .set(&DataKey::AnonymousVoteConfig(project_id), &vote_config);

    // // Emit event for anonymous voting setup
    event::AnonymousVotingSetup {
        project_id,
        judge,
        public_key,
    }
    .publish(&env);
}
