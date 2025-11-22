use soroban_sdk::{Address, BytesN, Env, String, Vec, contracttype, panic_with_error};

use crate::storage::{DataKey, error};

#[derive(Clone)]
#[contracttype]
pub struct Vote {
    pub account: Address,
    pub vote: bool,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Vote2 {
    //PublicVote(PublicVote),
    AnonymousVote(AnonymousVote),
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct AnonymousVoteConfig {
    pub vote_generator_point: BytesN<96>,
    pub seed_generator_point: BytesN<96>,
    pub public_key: String,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct AnonymousVote {
    pub address: Address,
    pub weight: u32,
    pub encrypted_seeds: Vec<String>,
    pub encrypted_votes: Vec<String>,
    pub commitments: Vec<BytesN<96>>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Badge {
    Developer = 10_000_000,
    Triage = 5_000_000,
    Community = 1_000_000,
    Verified = 500_000, // have a soroban domain
    Default = 1,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct VoteData {
    pub voting_ends_at: u64,
    //pub public_voting: bool,
    pub votes: Vec<Vote2>,
}

/// Get the anonymous voting configuration for a project.
///
/// # Arguments
/// * `env` - The environment object
/// * `project_id` - The project ID identifier
///
/// # Returns
/// * `types::AnonymousVoteConfig` - The anonymous voting configuration
///
/// # Panics
/// * If no anonymous voting configuration exists for the project
pub fn get_anonymous_voting_config(env: &Env, project_id: u32) -> AnonymousVoteConfig {
    env.storage()
        .instance()
        .get(&DataKey::AnonymousVoteConfig(project_id))
        .unwrap_or_else(|| {
            panic_with_error!(env, &error::Error::NoAnonymousVotingConfig);
        })
}

//            .set(&DataKey::AnonymousVoteConfig(project_id), &vote_config);
