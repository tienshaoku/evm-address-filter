use eyre::{Result, eyre};

pub fn read_csv(path: &str, header_name: &str) -> Result<(Vec<Vec<String>>, Vec<String>)> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;
    let mut full_csv: Vec<Vec<String>> = vec![];
    for rec in reader.records() {
        let rec = rec?;
        full_csv.push(rec.iter().map(|s| s.to_string()).collect());
    }

    reader = csv::Reader::from_path(path)?;
    let col_index = reader
        .headers()?
        .clone()
        .iter()
        .position(|x| x == header_name)
        .ok_or_else(|| eyre!("No such header: {}", header_name))?;

    let addresses = reader
        .records()
        .filter_map(|res| res.ok())
        .filter_map(|row| row.get(col_index).map(|v| v.to_string()))
        .collect();

    Ok((full_csv, addresses))
}

pub fn write_csv(path: &str, rows: &[Vec<String>]) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;
    for (i, row) in rows.iter().enumerate() {
        writer.write_record(row)?;

        if (i % 100) == 0 {
            writer.flush()?;
        }
    }
    writer.flush()?;
    Ok(())
}
