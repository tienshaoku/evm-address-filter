pub mod csv;
use dotenv::dotenv;
use std::env;

use crate::csv::read_csv_addresses;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::Address,
};
use eyre::Result;

pub async fn run() -> eyre::Result<()> {
    dotenv().ok();

    let addresses = read_csv_addresses("./input.csv", "address")?;
    let wallets = format_raw_addresses(addresses)?;

    let eth_rpc = env::var("ETH_RPC_URL")?;
    let bnb_rpc = env::var("BNB_RPC_URL")?;
    let rpc_urls = vec![eth_rpc, bnb_rpc];

    for rpc_url in &rpc_urls {
        for address in &wallets {
            let provider = Provider::<Http>::try_from(rpc_url)?;
            let balance = provider.get_balance(*address, None).await?;
            println!("{:?}", balance);
        }
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
