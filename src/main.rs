use evm_address_filter::run;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    run().await
}
