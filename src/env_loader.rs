use dotenv::dotenv;
use std::env;
use ethers::types::U64;

pub struct Env {
    pub infura_project_id: String,
    pub contract_address: String,
    pub network: String,
    pub block_number: U64,
}

impl Env {
    fn parse_from_env_to_u64(var_name: &str, error_message: &str) -> U64 {
        env::var(var_name)
            .expect(&format!("Expected {}", var_name))
            .parse::<u64>()
            .map(U64::from)
            .expect(error_message)
    }

    pub fn load_env() -> Self {
        dotenv().ok(); // Load .env file

        Self {
            infura_project_id: env::var("INFURA_PROJECT_ID").expect("Expected INFURA_PROJECT_ID"),
            contract_address: env::var("CONTRACT_ADDRESS").expect("Expected CONTRACT_ADDRESS"),
            network: env::var("NETWORK").expect("Expected NETWORK"),
            block_number: Self::parse_from_env_to_u64("BLOCK_NUMBER", "BLOCK_NUMBER must be a valid u64"),
        }
    }
}