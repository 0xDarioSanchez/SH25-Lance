use soroban_sdk::{Address, String, Vec, contracttype};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub url: String,  // link to VCS
    pub ipfs: String, // CID of the tansu.toml file with metadata
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Project {
    pub name: String,
    pub config: Config,
    pub maintainers: Vec<Address>,
}
