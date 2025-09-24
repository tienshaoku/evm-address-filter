pub mod csv;
use dotenv::dotenv;
use std::env;

use crate::csv::read_csv;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::Address,
    utils::parse_ether,
};
use eyre::Result;

const INPUT_PATH: &str = "./input.csv";
const OUTPUT_PATH: &str = "./output.csv";
const HEADER_NAME: &str = "address";
const ETH_THRESHOLD: &str = "0.002";
const BNB_THRESHOLD: &str = "0.01";

pub async fn run() -> eyre::Result<()> {
    dotenv().ok();

    let (mut full_csv, addresses) = read_csv(INPUT_PATH, HEADER_NAME)?;
    let wallets = format_raw_addresses(addresses)?;

    let eth_rpc = env::var("ETH_RPC_URL")?;
    let bnb_rpc = env::var("BNB_RPC_URL")?;
    let rpc_urls = vec![eth_rpc.clone(), bnb_rpc.clone()];

    let mut tmp = vec![];
    let mut res = vec![String::from("minor_onchain_asset")];
    let rpc_length = rpc_urls.len();
    for rpc_url in &rpc_urls {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let (threshold, col_index) = if *rpc_url == eth_rpc {
            (parse_ether(ETH_THRESHOLD)?, 0)
        } else {
            (parse_ether(BNB_THRESHOLD)?, 1)
        };
        for i in 0..wallets.len() {
            let balance = provider.get_balance(wallets[i], None).await?;

            if col_index != rpc_length - 1 {
                tmp.push(balance < threshold);
            } else {
                res.push((balance < threshold && tmp[i]).to_string());
            }
        }
    }
    for i in 0..full_csv.len() {
        full_csv[i].push(res[i].clone());
    }
    csv::write_csv(OUTPUT_PATH, &full_csv)?;

    Ok(())
}

fn format_raw_addresses(raw_addresses: Vec<String>) -> Result<Vec<Address>> {
    let mut addresses: Vec<Address> = Vec::new();
    for addr in raw_addresses {
        addresses.push(addr.parse()?)
    }
    Ok(addresses)
}
