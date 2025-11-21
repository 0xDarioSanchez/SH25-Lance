//use crate::{Tansu, TansuClient, domain_contract, outcomes_contract, types};
use soroban_sdk::testutils::{Address as _, Events};
use soroban_sdk::{
    Address, Bytes, BytesN, Env, Executable, IntoVal, Map, String, Symbol, Val, Vec, token, vec,
};

use crate::ProtocolContract;
use crate::contract::ProtocolContractClient;
use crate::storage::Dispute;

/// Helper function to compute commit hash off-chain
/// Hash = SHA256(vote_string || secret)
pub fn compute_commit_hash(env: &Env, vote: bool, secret: &Bytes) -> BytesN<32> {
    let vote_str = if vote { "true" } else { "false" };
    let mut data = Bytes::new(env);
    data.append(&Bytes::from_slice(env, vote_str.as_bytes()));
    data.append(secret);
    env.crypto().sha256(&data).into()
}

pub struct TestSetup {
    pub env: Env,
    pub contract: ProtocolContractClient<'static>,
    pub contract_id: Address,
    pub creator: Address,
    pub counterpart: Address,
    pub proof: String,
    //pub domain_id: Address,
    //pub outcomes_id: Address,
    pub token_stellar: token::StellarAssetClient<'static>,
    //pub grogu: Address,
    //pub mando: Address,
    pub contract_admin: Address,
    pub judge1: Address,
    pub judge2: Address,
    pub judge3: Address,
    pub project_id: u32,
    pub public_key: String,
    pub voting_ends_at: u64,
}

pub fn create_env() -> Env {
    let env = Env::default();
    env.mock_all_auths();
    env
}

pub fn create_test_data() -> TestSetup {
    let env = create_env();

    //let outcomes_id = env.register(outcomes_contract::WASM, ());

    //let domain_id = env.register(domain_contract::WASM, ());
    //let domain = domain_contract::Client::new(&env, &domain_id);

    let _adm = Address::generate(&env);
    //let node_rate: u128 = 100;
    //let min_duration: u64 = 31_536_000;
    /*let allowed_tlds: Vec<Bytes> = Vec::from_array(
        &env,
        [
            Bytes::from_slice(&env, b"xlm"),
            Bytes::from_slice(&env, b"stellar"),
            Bytes::from_slice(&env, b"wallet"),
            Bytes::from_slice(&env, b"dao"),
        ],
    );*/

    let issuer = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(issuer.clone());
    let token_client = token::TokenClient::new(&env, &sac.address());
    let token_stellar = token::StellarAssetClient::new(&env, &sac.address());

    /*domain.init(
        &adm,
        &node_rate,
        &token_client.address.clone(),
        &min_duration,
        &allowed_tlds,
    );*/

    let contract_admin = Address::generate(&env);
    let contract_id = env.register(ProtocolContract, (&contract_admin, token_client.address));
    let contract = ProtocolContractClient::new(&env, &contract_id);

    /*contract.pause(&contract_admin, &false);

    let wasm_hash = match domain_id.executable().unwrap() {
        Executable::Wasm(wasm) => wasm,
        _ => panic!(),
    };

    let new_domain = types::Contract {
        address: domain_id.clone(),
        wasm_hash: Some(wasm_hash.clone()),
    };
    contract.set_domain_contract(&contract_admin, &new_domain);

    let new_collateral = types::Contract {
        address: sac.address(),
        wasm_hash: None,
    };
    contract.set_collateral_contract(&contract_admin, &new_collateral);

    let grogu = Address::generate(&env);
    let mando = Address::generate(&env);*/
    let creator = Address::generate(&env);
    let counterpart = Address::generate(&env);
    let judge1 = Address::generate(&env);
    let judge2 = Address::generate(&env);
    let judge3 = Address::generate(&env);
    let proof = String::from_str(&env, "test proof 1");
    let project_id = 1;
    let public_key = String::from_str(&env, "test public key 1");
    let voting_ends_at = env.ledger().timestamp() + 3600 * 24 * 2;

    TestSetup {
        env,
        contract,
        contract_id,
        creator,
        counterpart,
        //domain_id,
        //outcomes_id,
        token_stellar,
        //grogu,
        //mando,
        contract_admin,
        proof,
        judge1,
        judge2,
        judge3,
        project_id,
        public_key,
        voting_ends_at,
    }
}

pub fn init_contract(setup: &TestSetup) -> Dispute {
    let _name = String::from_str(&setup.env, "tansu");
    let _url = String::from_str(&setup.env, "github.com/tansu");
    let _ipfs = String::from_str(&setup.env, "2ef4f49fdd8fa9dc463f1f06a094c26b88710990");
    //let maintainers = vec![&setup.env, setup.grogu.clone(), setup.mando.clone()];

    let genesis_amount: i128 = 1_000_000_000 * 10_000_000;
    //setup.token_stellar.mint(&setup.grogu, &genesis_amount);
    //setup.token_stellar.mint(&setup.mando, &genesis_amount);

    let dispute = setup.contract.create_dispute(
        &setup.project_id,
        &setup.public_key,
        &setup.creator,
        &setup.counterpart,
        &setup.proof,
        &setup.voting_ends_at,
    );

    let all_events = setup.env.events().all();
    assert_eq!(
        all_events,
        vec![
            &setup.env,
            (
                setup.contract_id.clone(),
                (
                    Symbol::new(&setup.env, "anonymous_dispute_setup"),
                    setup.project_id.clone()
                )
                    .into_val(&setup.env),
                Map::<Symbol, Val>::from_array(
                    &setup.env,
                    [
                        (
                            Symbol::new(&setup.env, "creator"),
                            setup.creator.clone().into_val(&setup.env)
                        ),
                        (
                            Symbol::new(&setup.env, "public_key"),
                            setup.public_key.into_val(&setup.env)
                        ),
                    ],
                )
                .into_val(&setup.env),
            ),
        ]
    );

    assert_eq!(dispute.vote_data.votes, vec![&setup.env]);

    setup.contract.new_voter(&setup.judge1);
    setup
        .contract
        .register_to_vote(&setup.judge1, &dispute.dispute_id);

    dispute
}
