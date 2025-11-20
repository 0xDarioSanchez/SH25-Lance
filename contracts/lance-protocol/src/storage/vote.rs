use soroban_sdk::{Address, contracttype};

#[derive(Clone)]
#[contracttype]
pub struct Vote {
    pub account: Address,
    pub vote: bool,
}
