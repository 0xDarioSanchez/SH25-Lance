use soroban_sdk::{contract, contractimpl, Bytes, Env, Address, String, Vec};
use crate::storage::{Voter, Dispute, error::Error};
use crate::methods::{
    initialize::initialize,
    balance::{get_balance, redeem},
    dispute::create_dispute,
    vote::{register_to_vote, commit_vote, reveal_votes},
};
use crate::storage::voter::{get_voter, set_voter};

pub trait ProtocolContractTrait {
    fn __constructor(env: Env, admin: Address, token: Address) -> Result<(), Error>;

    fn new_voter(
        env: Env,
        user: Address,
        // personal_data: Option<String>,
    ) -> Result<(), Error>;

    fn get_user(env: Env, user: Address) -> Result<Voter, Error>;

    fn create_dispute(
        env: &Env,
        creator: Address,
        counterpart: Address,
        proof: String,
    ) -> Result<Dispute, Error>;

    fn get_balance(env: &Env, employee: Address) -> i128;

    fn redeem(
        env: &Env,
        employee: Address,
    ) -> Result<i128, Error>;

    fn register_to_vote(
        env: &Env, 
        creator: Address, 
        dispute_id: u32
    ) -> Result<Dispute, Error>;

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
    
    fn redeem(
        env: &Env,
        employee: Address,
    ) -> Result<i128, Error> {
        redeem(env, employee)   
    }

    fn register_to_vote(
        env: &Env, 
        creator: Address, 
        dispute_id: u32
    ) -> Result<Dispute, Error> {
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