use crate::events::event;
use crate::storage::error;
use crate::storage::project::Project;
use crate::storage::voter::{get_voter, set_voter};
use crate::storage::{DataKey, Dispute, Voter, error::Error};
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
use soroban_sdk::crypto::bls12_381::G1Affine;
use soroban_sdk::{
    Address, Bytes, BytesN, Env, String, U256, Vec, contract, contractimpl, panic_with_error,
};

pub trait ProtocolContractTrait {
    fn __constructor(env: Env, admin: Address, token: Address) -> Result<(), Error>;

    fn new_voter(
        env: Env,
        user: Address,
        // personal_data: Option<String>,
    ) -> Result<(), Error>;

    fn get_user(env: Env, user: Address) -> Result<Voter, Error>;

    fn anonymous_voting_setup(env: Env, maintainer: Address, project_id: u32, public_key: String);

    fn build_commitments_from_votes(
        env: Env,
        project_id: u32,
        votes: Vec<u128>,
        seeds: Vec<u128>,
    ) -> Vec<BytesN<96>>;

    fn create_dispute(
        env: &Env,
        project_id: u32,
        public_key: String,
        creator: Address,
        counterpart: Address,
        proof: String,
        voting_ends_at: u64,
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
        admin::anonymous_voting_setup(env, judge, project_id, public_key)
    }

    /// Build vote commitments from votes and seeds for anonymous voting.
    ///
    /// Creates BLS12-381 commitments for each vote using the formula:
    /// C = g·vote + h·seed where g and h are generator points on BLS12-381.
    ///
    /// Note: This function does not consider voting weights, which are applied
    /// during the tallying phase. Calling this on the smart contract would reveal
    /// the votes and seeds, so it must be run either in simulation or client-side.
    ///
    /// # Arguments
    /// * `env` - The environment object
    /// * `project_key` - Unique identifier for the project
    /// * `votes` - Vector of vote choices (0=approve, 1=reject, 2=abstain)
    /// * `seeds` - Vector of random seeds for each vote
    ///
    /// # Returns
    /// * `Vec<BytesN<96>>` - Vector of vote commitments (one per vote)
    ///
    /// # Panics
    /// * If no anonymous voting configuration exists for the project
    fn build_commitments_from_votes(
        env: Env,
        dispute_id: u32,
        votes: Vec<u128>,
        seeds: Vec<u128>,
    ) -> Vec<BytesN<96>> {
        // Validate that votes and seeds have the same length
        if votes.len() != seeds.len() {
            panic_with_error!(&env, &error::Error::TallySeedError);
        }

        let vote_config = vote::get_anonymous_voting_config(env.clone(), dispute_id);

        let bls12_381 = env.crypto().bls12_381();
        let seed_generator_point = G1Affine::from_bytes(vote_config.seed_generator_point);
        let vote_generator_point = G1Affine::from_bytes(vote_config.vote_generator_point);

        let mut commitments = Vec::new(&env);
        for (vote_, seed_) in votes.iter().zip(seeds.iter()) {
            let vote_: U256 = U256::from_u128(&env, vote_);
            let seed_: U256 = U256::from_u128(&env, seed_);
            let seed_point_ = bls12_381.g1_mul(&seed_generator_point, &seed_.into());
            let vote_point_ = bls12_381.g1_mul(&vote_generator_point, &vote_.into());

            commitments.push_back(bls12_381.g1_add(&vote_point_, &seed_point_).to_bytes());
        }
        commitments
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
        project_id: u32,
        public_key: String,
        creator: Address,
        counterpart: Address,
        proof: String,
        voting_ends_at: u64,
    ) -> Result<Dispute, Error> {
        create_dispute(
            env,
            project_id,
            public_key,
            creator,
            counterpart,
            proof,
            voting_ends_at,
        )
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
