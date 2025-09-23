pub mod csv;
use crate::csv::read_csv_addresses;

pub fn run() -> anyhow::Result<()> {
    let addresses = read_csv_addresses("./input.csv", "address")?;
    Ok(())
}
