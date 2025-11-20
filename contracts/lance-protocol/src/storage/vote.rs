use soroban_sdk::{Address, BytesN, String, contracttype};

#[derive(Clone)]
#[contracttype]
pub struct Vote {
    pub account: Address,
    pub vote: bool,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct AnonymousVoteConfig {
    pub vote_generator_point: BytesN<96>,
    pub seed_generator_point: BytesN<96>,
    pub public_key: String,
}
