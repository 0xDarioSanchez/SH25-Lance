use soroban_sdk::{Address, Env, panic_with_error};

use crate::storage::{self, DataKey, error};

pub(crate) fn has_admin(env: &Env) -> bool {
    let key = DataKey::Admin;

    env.storage().instance().has(&key)
}

pub(crate) fn set_admin(env: &Env, admin: &Address) {
    let key = DataKey::Admin;

    env.storage().instance().set(&key, admin);
}

pub(crate) fn auth_maintainers(
    env: &Env,
    maintainer: &Address,
    project_id: u32,
) -> storage::Dispute {
    maintainer.require_auth();
    //let project_key_ = types::ProjectKey::Key(project_key.clone());
    if let Some(dispute) = env
        .storage()
        .instance()
        .get::<DataKey, storage::Dispute>(&DataKey::Disputes(project_id))
    {
        if !dispute.able_to_vote.contains(maintainer) && !dispute.voters.contains(maintainer) {
            //QUESTION: should we panic here?
            panic_with_error!(&env, &error::Error::UnauthorizedSigner);
        }
        dispute
    } else {
        panic_with_error!(&env, &error::Error::InvalidKey)
    }
}
