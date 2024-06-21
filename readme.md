
# query_contract_by_block

[![Rust CI](https://github.com/Amarlanda/query_contract_by_block/actions/workflows/rust.yml/badge.svg)](https://github.com/Amarlanda/query_contract_by_block/actions/workflows/rust.yml)

This repository contains the code to query a contract by block without the contract's ABI.

The purpose of this repository is to ensure that API keys are not exposed, while still demonstrating their usage in the workflow for running the code locally.

## Setup

To run this project, configure the following environment variables:

```sh
INFURA_PROJECT_ID=xxxxx
ETHERSCAN_API_KEY=xxxxx
CONTRACT_ADDRESS=0x9b7FD6FF5e427F8470E1da652f21A79Bed318f38
NETWORK=sepolia
BLOCK_NUMBER=6147190
```

## Usage

To run the project, use the following command:

```sh
cargo run --release
```


## Review the Results
If you would like to view the code output without running it, please click the link below.

## Output

To access the GitHub workflow output, log in to GitHub and click on this [link](https://github.com/Amarlanda/query_contract_by_block/actions/runs/9610611684/job/26507524634#step:7:287)
