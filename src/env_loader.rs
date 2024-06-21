use dotenv::dotenv;
use std::env;
use ethers::types::U64;

pub struct Env {
    pub infura_project_id: String,
    pub contract_address: String,
    pub network: String,
    pub block_number: U64,
}

pub fn load_env() -> Env {
    dotenv().ok(); // Load .env file

    Env {
        infura_project_id: env::var("INFURA_PROJECT_ID").expect("Expected INFURA_PROJECT_ID"),
        contract_address: env::var("CONTRACT_ADDRESS").expect("Expected CONTRACT_ADDRESS"),
        network: env::var("NETWORK").expect("Expected NETWORK"),
        block_number: U64::from(env::var("BLOCK_NUMBER").unwrap().parse::<u64>().unwrap()),
    }
}
