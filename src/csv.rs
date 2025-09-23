use eyre::{Result, eyre};

pub fn read_csv_addresses(path: &str, header_name: &str) -> Result<Vec<String>> {
    let mut reader = csv::Reader::from_path(path)?;

    let col_index = reader
        .headers()?
        .clone()
        .iter()
        .position(|x| x == header_name)
        .ok_or_else(|| eyre!("No such header: {}", header_name))?;

    let arr = reader
        .records()
        .filter_map(|res| res.ok())
        .filter_map(|row| row.get(col_index).map(|v| v.to_string()))
        .collect();
    Ok(arr)
}
