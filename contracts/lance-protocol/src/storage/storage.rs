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
}
