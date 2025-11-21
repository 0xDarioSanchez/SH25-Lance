use soroban_sdk::contracttype;

#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum DisputeStatus {
    OPEN,
    VOTING, 
    EXECUTED,
    FINISHED,
}