use soroban_sdk::{Address, Env, Symbol};

use crate::methods::dispute::create_dispute;

pub(crate) fn created_dispute(env: &Env, creator: &Address, service_id: &u32) {
    let topics = (Symbol::new(env,"created_dispute"), service_id);
    let data = (creator, service_id);
    env.events().publish(topics, data);
}

pub(crate) fn created_service(env: &Env, creator: &Address, service_id: &u32) {
    let topics: (Symbol, &u32) = (Symbol::new(env,"created_service"), service_id);
    let data = (creator, service_id);
    env.events().publish(topics, data);
}