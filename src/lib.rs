pub mod csv;
use dotenv::dotenv;
use std::env;

use crate::csv::read_csv_addresses;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::Address,
    utils::parse_ether,
};
use eyre::Result;

const INPUT_PATH: &str = "./input.csv";
const HEADER_NAME: &str = "address";
const ETH_THRESHOLD: &str = "0.002";
const BNB_THRESHOLD: &str = "0.01";

pub async fn run() -> eyre::Result<()> {
    dotenv().ok();

    let addresses = read_csv_addresses(INPUT_PATH, HEADER_NAME)?;
    let wallets = format_raw_addresses(addresses)?;

    let eth_rpc = env::var("ETH_RPC_URL")?;
    let bnb_rpc = env::var("BNB_RPC_URL")?;
    let rpc_urls = vec![eth_rpc.clone(), bnb_rpc.clone()];

    let mut res = vec![];
    for i in 0..rpc_urls.len() {
        let mut current_rol = vec![];
        let threshold = if rpc_urls[i] == eth_rpc {
            parse_ether(ETH_THRESHOLD)?
        } else {
            parse_ether(BNB_THRESHOLD)?
        };
        for address in &wallets {
            let provider = Provider::<Http>::try_from(rpc_urls[i].clone())?;
            let balance = provider.get_balance(*address, None).await?;

            current_rol.push(balance < threshold);
        }
        res.push(current_rol);
    }

    Ok(())
}

fn format_raw_addresses(raw_addresses: Vec<String>) -> Result<Vec<Address>> {
    let mut addresses: Vec<Address> = Vec::new();
    for addr in raw_addresses {
        addresses.push(addr.parse()?)
    }
    Ok(addresses)
}
