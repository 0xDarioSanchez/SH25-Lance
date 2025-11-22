use soroban_sdk::contracttype;

#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum ServiceStatus {
    CREATED,
    ACCEPTED,
    WAITING,
    DISPUTING,
}
