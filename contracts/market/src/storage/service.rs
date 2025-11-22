use soroban_sdk::{contracttype, Address, String, Env};
use crate::storage::{error::Error, service_status::ServiceStatus, storage::DataKey};

#[derive(Clone)]
#[contracttype]
pub struct Service {
    pub id: u32,
    pub metadata: Option<String>,
    pub employee: Address,
    pub employer: Address,
    pub duration: u64,
    pub started_moment: u64,
    pub status: ServiceStatus,
    pub current_milestone: u32,
    pub milestone_payment: i128, 
}

pub fn set_service(env: &Env, service_id: u32, service: Service) {
    let key = DataKey::Services(service_id);

    env.storage().instance().set(&key, &service)
}

pub fn get_service(env: &Env, service_id: u32) -> Result<Service, Error> {
    let key = DataKey::Services(service_id);

    env.storage()
        .instance()
        .get(&key)
        .ok_or(Error::ServiceNotFound)
}

pub fn remove_service(env: &Env, service_id: u32) {
    let key = DataKey::Services(service_id);
    env.storage().instance().remove(&key);  
}