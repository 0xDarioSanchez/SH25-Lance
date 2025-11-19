use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub struct Vote {
    pub account: Address,
    pub vote: bool,
}
