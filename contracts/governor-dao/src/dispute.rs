use crate::{
    errors::GovernorError,
    storage,
};
use soroban_sdk::{panic_with_error, Address, Env, String};

const TIME_ONE_DAY: u64 = 24 * 60 * 60;

// Import the lance-protocol contract client
mod lance_protocol {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/lance_protocol.optimized.wasm"
    );
}

/// Create a dispute for a proposal in the lance-protocol contract.
/// This enables proposals to be disputed and resolved through anonymous voting.
pub fn create_dispute_for_proposal(
    env: &Env,
    creator: Address,
    proposal_id: u32,
    proof: String,
) -> u32 {
    creator.require_auth();
    storage::extend_instance(env);

    // Verify the proposal exists
    let proposal_data = storage::get_proposal_data(env, proposal_id)
        .unwrap_or_else(|| panic_with_error!(env, GovernorError::NonExistentProposalError));

    // Get the lance-protocol contract address from storage
    let lance_protocol_contract = env
        .storage()
        .instance()
        .get::<String, Address>(&String::from_str(env, "LanceProtocolContract"))
        .unwrap_or_else(|| panic_with_error!(env, GovernorError::InvalidKey));

    // Calculate voting end time (e.g., 7 days from now)
    let voting_ends_at = env.ledger().timestamp() + (7 * TIME_ONE_DAY);

    // Determine counterpart (for governance, it's typically the proposal creator)
    let counterpart = proposal_data.creator.clone();

    // Call lance-protocol contract to create the dispute
    let lance_client = lance_protocol::Client::new(env, &lance_protocol_contract);
    let dispute = lance_client.create_dispute(
        &proposal_id,                    // project_id (using proposal_id as project_id)
        &creator,                        // creator (the one creating the dispute)
        &counterpart,                    // counterpart (the proposal creator)
        &proof,                          // proof/evidence
        &voting_ends_at,                 // voting_ends_at
        &env.current_contract_address(), // called_contract (this governor contract)
    );

    // Return the dispute_id from lance-protocol
    dispute.dispute_id
}
