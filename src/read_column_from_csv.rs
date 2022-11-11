use std::path::Path;

use csv::Result;

pub fn read_column_from_csv<P: AsRef<Path>>(path: P, nth_column: usize) -> Result<Vec<f64>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(&path)?;

    let data = rdr
        .records()
        .map(|record| {
            record
                .expect("Failed to read string record")
                .get(nth_column)
                .expect(&format!(
                    "There is no {}th column in {}",
                    nth_column,
                    &path.as_ref().to_string_lossy()
                ))
                .parse::<f64>()
                .expect(&format!(
                    "Couldn't parse as f64 in {}",
                    &path.as_ref().to_string_lossy()
                ))
        })
        .collect::<Vec<_>>();

    Ok(data)
}
