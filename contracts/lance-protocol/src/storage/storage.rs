use soroban_sdk::{Address, contracttype};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Voters(Address),
    Disputes(u32),
    DisputeId,
    Balances(Address),
    AnonymousVoteConfig(u32),
    RewardClaimed(u32, Address), // (dispute_id, voter_address)
}
