use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    BlendPool,
    TotalPrincipal,
    Users(Address),
    Services(u32),
    Disputes(u32),
    DisputeId,
    Balances(Address),
    LanceProtocolContract,
}
