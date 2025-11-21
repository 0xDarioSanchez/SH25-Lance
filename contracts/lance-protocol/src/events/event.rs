use soroban_sdk::{Address, String, contractevent};

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnonymousVotingSetup {
    #[topic]
    pub project_id: u32,
    pub judge: Address,
    pub public_key: String,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnonymousDisputeSetup {
    #[topic]
    pub project_id: u32,
    pub creator: Address,
    pub public_key: String,
}
