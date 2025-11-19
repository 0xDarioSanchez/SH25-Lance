use soroban_sdk::{contractimpl, Bytes, BytesN, Env, Address, Error, String};
use crate::storage::{Voter, Dispute};
pub trait ProtocolContractTrait {
    fn __constructor(env: Env, admin: Address, token: Address, blend_pool: Address) -> Result<(), Error>;

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
        id: u32,
        reason: String,
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
        commit_hash: BytesN<32>,
    ) -> Result<Dispute, Error>;

    fn reveal_vote(
        env: &Env,
        voter: Address,
        dispute_id: u32,
        vote: bool,
        secret: Bytes,
    ) -> Result<Dispute, Error>;

    // fn vote(
    //     env: &Env, 
    //     creator: Address, 
    //     dispute_id: u32, 
    //     vote: Vote
    // ) -> Result<Dispute, Error>;
}



#[contract]
pub struct ProtocolContract;

#[contractimpl]
impl ProtocolContractTrait for ProtocolContract {
    fn __constructor(env: Env, admin: Address, token: Address, blend_pool: Address) -> Result<(), Error> {
        initialize(&env, admin, token, blend_pool)
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
        id: u32,
        proof: String,
    ) -> Result<Dispute, Error> {
        create_dispute(env, creator, counterpart, id, proof)
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
        commit_hash: BytesN<32>,
    ) -> Result<Dispute, Error> {
        commit_vote(env, voter, dispute_id, commit_hash)
    }

    fn reveal_vote(
        env: &Env,
        voter: Address,
        dispute_id: u32,
        vote: bool,
        secret: Bytes,
    ) -> Result<Dispute, Error> {
        reveal_vote(env, voter, dispute_id, vote, secret)
    }

    // fn vote(
    //     env: &Env, 
    //     creator: Address, 
    //     dispute_id: u32, 
    //     user_vote: Vote
    // ) -> Result<Dispute, Error> {
    //     vote(env, creator, dispute_id, user_vote)
    // } 
}