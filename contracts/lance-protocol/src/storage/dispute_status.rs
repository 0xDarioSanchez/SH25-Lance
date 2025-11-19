use soroban_sdk::contracttype;

#[derive(Clone, PartialEq)]
#[contracttype]
pub enum DisputeStatus {
    OPEN,
    VOTING, 
    EXECUTED,
    FINISHED,
}