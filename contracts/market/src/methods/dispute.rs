use crate::events::event::created_dispute;
use crate::storage::{error::Error, service::*, service_status::ServiceStatus, storage::DataKey};
use soroban_sdk::{Address, Env, String};

const TIME_ONE_DAY: u64 = 24 * 60 * 60;

// Import the lance-protocol contract client
mod lance_protocol {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/lance_protocol.optimized.wasm"
    );
}

pub fn create_dispute(
    env: &Env,
    creator: Address,
    service_id: u32,
    proof: String,
) -> Result<u32, Error> {
    creator.require_auth();

    let mut service = get_service(env, service_id)?;
    let employee = service.employee.clone();
    let employer = service.employer.clone();

    if creator != employer {
        return Err(Error::NotAuthorized);
    }
    if service.status != ServiceStatus::ACCEPTED {
        return Err(Error::InvalidServiceStatus);
    }
    if service.status == ServiceStatus::DISPUTING {
        return Err(Error::DisputeAlreadyCreated);
    }

    // Get the lance-protocol contract address from storage
    let lance_protocol_contract = env
        .storage()
        .instance()
        .get::<DataKey, Address>(&DataKey::LanceProtocolContract)
        .ok_or(Error::InvalidKey)?;

    // Calculate voting end time (e.g., 7 days from now)
    let voting_ends_at = env.ledger().timestamp() + (7 * TIME_ONE_DAY);

    // Call lance-protocol contract to create the dispute
    let lance_client = lance_protocol::Client::new(env, &lance_protocol_contract);
    let dispute = lance_client.create_dispute(
        &service_id,                     // project_id (using service_id as project_id)
        &employer,                       // creator
        &employee,                       // counterpart
        &proof,                          // proof
        &voting_ends_at,                 // voting_ends_at
        &env.current_contract_address(), // called_contract (this market contract)
        &service.milestone_payment,      // amount of the service
    );

    // Update service status to DISPUTING
    service.status = ServiceStatus::DISPUTING;
    set_service(env, service_id, service.clone());

    // Emit event
    created_dispute(env, &creator, &service_id);

    // Return the dispute_id from lance-protocol
    Ok(dispute.dispute_id)
}

pub fn update_dispute(
    env: &Env,
    service_id: u32,
    dispute_id: u32,
    proof: String,
) -> Result<(), Error> {
    let service = get_service(env, service_id)?;
    let employee = service.employee.clone();

    employee.require_auth();

    if service.status != ServiceStatus::DISPUTING {
        return Err(Error::InvalidDisputeStatus);
    }

    // Get the lance-protocol contract address
    let lance_protocol_contract = env
        .storage()
        .instance()
        .get::<DataKey, Address>(&DataKey::LanceProtocolContract)
        .ok_or(Error::InvalidKey)?;

    // Call lance-protocol to get the dispute
    let lance_client = lance_protocol::Client::new(env, &lance_protocol_contract);
    let dispute = lance_client.get_dispute(&dispute_id);

    // Verify the employee is the counterpart in the dispute
    if dispute.counterpart != employee {
        return Err(Error::NotAuthorized);
    }

    // TODO: Add method to lance-protocol to update counterpart proof
    // For now, this would require adding an update_dispute_proof function to lance-protocol

    Ok(())
}
