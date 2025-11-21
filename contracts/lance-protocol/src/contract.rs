use crate::events::event;
use crate::methods::dispute::execute;
use crate::storage::dispute::get_dispute;
use crate::storage::dispute_status::DisputeStatus;
use crate::storage::error;
use crate::storage::project::Project;
use crate::storage::vote::Vote2;
use crate::storage::voter::{get_voter, set_voter};
use crate::storage::{DataKey, Dispute, Voter, error::Error};
use crate::{
    methods::{
        admin::anonymous_voting_setup,
        balance::{get_balance, redeem},
        dispute::create_dispute,
        initialize::initialize,
        vote::{build_commitments_from_votes, commit_vote, register_to_vote, reveal_votes, vote},
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

    fn get_dispute(env: Env, dispute_id: u32) -> Result<Dispute, Error>;

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
        called_contract: Address,
    ) -> Result<Dispute, Error>;

    fn get_balance(env: &Env, employee: Address) -> i128;

    fn redeem(env: &Env, employee: Address) -> Result<i128, Error>;

    fn register_to_vote(env: &Env, creator: Address, dispute_id: u32) -> Result<Dispute, Error>;

    fn commit_vote(
        env: &Env,
        voter: Address,
        dispute_id: u32,
        commit_hash: BytesN<32>,
    ) -> Result<Dispute, Error>;

    fn reveal_votes(
        env: &Env,
        creator: Address,
        dispute_id: u32,
        votes: Vec<bool>,
        secrets: Vec<Bytes>,
    ) -> Result<Dispute, Error>;

    fn vote(env: Env, voter: Address, dispute_id: u32, vote_data: Vote2);

    fn execute(
        env: Env,
        maintainer: Address,
        project_id: u32,
        dispute_id: u32,
        tallies: Option<Vec<u128>>,
        seeds: Option<Vec<u128>>,
    ) -> DisputeStatus;
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
        anonymous_voting_setup(env, judge, project_id, public_key)
    }

    fn build_commitments_from_votes(
        env: Env,
        dispute_id: u32,
        votes: Vec<u128>,
        seeds: Vec<u128>,
    ) -> Vec<BytesN<96>> {
        build_commitments_from_votes(env, dispute_id, votes, seeds)
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

    fn get_dispute(env: Env, dispute_id: u32) -> Result<Dispute, Error> {
        get_dispute(&env, dispute_id)
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
        called_contract: Address,
    ) -> Result<Dispute, Error> {
        create_dispute(
            env,
            project_id,
            public_key,
            creator,
            counterpart,
            proof,
            voting_ends_at,
            called_contract,
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
        commit_hash: BytesN<32>,
    ) -> Result<Dispute, Error> {
        commit_vote(env, voter, dispute_id, commit_hash)
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

    fn vote(env: Env, voter: Address, dispute_id: u32, vote_data: Vote2) {
        vote(env, voter, dispute_id, vote_data);
    }

    fn execute(
        env: Env,
        maintainer: Address,
        project_id: u32,
        dispute_id: u32,
        tallies: Option<Vec<u128>>,
        seeds: Option<Vec<u128>>,
    ) -> DisputeStatus {
        execute(env, maintainer, project_id, dispute_id, tallies, seeds)
    }
}
