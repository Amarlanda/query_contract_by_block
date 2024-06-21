use ethers::{
  abi::{ParamType, Token},
  prelude::*,
  types::transaction::eip2718::TypedTransaction,
  utils::keccak256,
};
use std::{convert::TryFrom, sync::Arc};
use tokio::main;

mod env_loader;
use env_loader:: Env;

async fn connect_to_network(env: &Env) -> Result<Arc<Provider<Http>>, Box<dyn std::error::Error>> {
  let infura_url = format!("https://{}.infura.io/v3/{}", env.network, env.infura_project_id);
  println!("Connecting to network: {}", env.network);
  Ok(Arc::new(Provider::<Http>::try_from(infura_url)?))
}

async fn check_connection(client: &Arc<Provider<Http>>, env: &Env) -> Result<(), Box<dyn std::error::Error>> {
  let block_number: U64 = client.get_block_number().await?;
  println!("Connected to {} network at block number: {}", env.network, block_number);
  Ok(())
}

fn parse_contract_address(contract_address: &str) -> Result<Address, Box<dyn std::error::Error>> {
  println!("Parsing contract address...");
  let address = contract_address.parse::<Address>()?;
  println!("Parsed contract address: {}", address);
  Ok(address)
}

fn encode_function_and_argument(function_signature: &str, argument: U256) -> (Vec<u8>, Vec<u8>) {
  println!("Encoding function signature and argument...");
  let function_selector = keccak256(function_signature.as_bytes())[..4].to_vec();
  let encoded_argument = ethers::abi::encode(&[Token::Uint(argument)]);
  println!("Encoded function selector: 0x{}", hex::encode(&function_selector));
  println!("Encoded argument: 0x{}", hex::encode(&encoded_argument));
  (function_selector, encoded_argument)
}

fn create_data_payload(function_selector: &[u8], encoded_argument: &[u8]) -> Vec<u8> {
  println!("Creating data payload...");
  let mut d = Vec::with_capacity(4 + 32);
  d.extend_from_slice(function_selector);
  d.extend_from_slice(encoded_argument);
  println!("Data payload: 0x{}", hex::encode(&d));
  d
}

fn create_transaction_request(contract_address: Address, data_payload: &[u8]) -> Eip1559TransactionRequest {
  println!("Creating transaction request...");
  let request = Eip1559TransactionRequest::new()
      .to(contract_address)
      .data(Bytes::from(data_payload.to_vec()))
      .from(Address::zero());
  println!("Created transaction request");
  request
}

async fn send_transaction(client: &Provider<Http>, transaction_request: Eip1559TransactionRequest, block_number: U64) -> Result<Bytes, Box<dyn std::error::Error>> {
  println!("Sending transaction...");
  let typed_tx = TypedTransaction::Eip1559(transaction_request);
  let result = client.call(&typed_tx, Some(block_number.into()))
      .await
      .map_err(|e| e.into()); // Convert ProviderError to Box<dyn std::error::Error>
  println!("Transaction sent. Awaiting result...");
  result
}

fn decode_result(result: &Bytes) -> Result<U256, Box<dyn std::error::Error>> {
  println!("Decoding result...");
  let decoded_result: U256 = ethers::abi::decode(&[ParamType::Uint(256)], result)?
      .pop()
      .expect("Expected at least one element")
      .into_uint()
      .expect("Expected Uint");
  println!("Decoded result: {}", decoded_result);
  Ok(decoded_result)
}

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Loading environment variables...
  let env = Env::load_env();

  // Connecting to the network...
  let client = connect_to_network(&env).await?;

  // Checking the connection...
  check_connection(&client, &env).await?;

  // Preparing contract call...
  let contract_address = parse_contract_address(&env.contract_address)?;

  // Encoding function and arguments...
  let (function_selector, encoded_argument) = encode_function_and_argument("gsr_query(uint256)", U256::from(1));

  // Creating data payload...
  let data_payload = create_data_payload(&function_selector, &encoded_argument);

  // Creating transaction request...
  let transaction_request = create_transaction_request(contract_address, &data_payload);

  // Sending the transaction...
  let result = send_transaction(&client, transaction_request, env.block_number).await?;

  // Decoding the result...
 decode_result(&result)?;

  Ok(())
}
