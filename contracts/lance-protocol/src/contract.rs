use crate::events::event;
use crate::storage::voter::{get_voter, set_voter};
use crate::storage::{Dispute, Voter, error::Error};
use crate::{
    methods::{
        admin,
        balance::{get_balance, redeem},
        dispute::create_dispute,
        initialize::initialize,
        vote::{commit_vote, register_to_vote, reveal_votes},
    },
    storage::vote,
};
use soroban_sdk::{Address, Bytes, Env, String, Vec, contract, contractimpl};

pub trait ProtocolContractTrait {
    fn __constructor(env: Env, admin: Address, token: Address) -> Result<(), Error>;

    fn new_voter(
        env: Env,
        user: Address,
        // personal_data: Option<String>,
    ) -> Result<(), Error>;

    fn get_user(env: Env, user: Address) -> Result<Voter, Error>;

    fn anonymous_voting_setup(env: Env, maintainer: Address, project_id: u32, public_key: String);

    fn create_dispute(
        env: &Env,
        creator: Address,
        counterpart: Address,
        proof: String,
    ) -> Result<Dispute, Error>;

    fn get_balance(env: &Env, employee: Address) -> i128;

    fn redeem(env: &Env, employee: Address) -> Result<i128, Error>;

    fn register_to_vote(env: &Env, creator: Address, dispute_id: u32) -> Result<Dispute, Error>;

    fn commit_vote(
        env: &Env,
        voter: Address,
        dispute_id: u32,
        vote: bool,
        secret: Bytes,
    ) -> Result<Dispute, Error>;

    fn reveal_votes(
        env: &Env,
        creator: Address,
        dispute_id: u32,
        votes: Vec<bool>,
        secrets: Vec<Bytes>,
    ) -> Result<Dispute, Error>;
}

#[contract]
pub struct ProtocolContract;

#[contractimpl]
impl ProtocolContractTrait for ProtocolContract {
    fn __constructor(env: Env, admin: Address, token: Address) -> Result<(), Error> {
        initialize(&env, admin, token)
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
    fn anonymous_voting_setup(env: Env, judge: Address, project_id: u32, public_key: String) {
        admin::auth_maintainers(&env, &judge, project_id);

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

        env.storage().instance().set(&project_id, &vote_config);

        // Emit event for anonymous voting setup
        event::AnonymousVotingSetup {
            project_id,
            judge,
            public_key,
        }
        .publish(&env);
    }

    fn new_voter(
        env: Env,
        user: Address,
        // personal_data: Option<String>,
    ) -> Result<(), Error> {
        set_voter(&env, user);
        Ok(())
    }

    fn get_user(env: Env, user: Address) -> Result<Voter, Error> {
        get_voter(&env, user)
    }

    fn get_balance(env: &Env, employee: Address) -> i128 {
        get_balance(env, &employee)
    }

    fn create_dispute(
        env: &Env,
        creator: Address,
        counterpart: Address,
        proof: String,
    ) -> Result<Dispute, Error> {
        create_dispute(env, creator, counterpart, proof)
    }

    fn redeem(env: &Env, employee: Address) -> Result<i128, Error> {
        redeem(env, employee)
    }

    fn register_to_vote(env: &Env, creator: Address, dispute_id: u32) -> Result<Dispute, Error> {
        register_to_vote(env, creator, dispute_id)
    }

    fn commit_vote(
        env: &Env,
        voter: Address,
        dispute_id: u32,
        vote: bool,
        secret: Bytes,
    ) -> Result<Dispute, Error> {
        commit_vote(env, voter, dispute_id, vote, secret)
    }

    fn reveal_votes(
        env: &Env,
        creator: Address,
        dispute_id: u32,
        votes: Vec<bool>,
        secrets: Vec<Bytes>,
    ) -> Result<Dispute, Error> {
        reveal_votes(env, creator, dispute_id, votes, secrets)
    }
}
