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
                .unwrap_or_else(|| {
                    panic!(
                        "There is no {}th column in {}",
                        nth_column,
                        &path.as_ref().to_string_lossy()
                    )
                })
                .parse::<f64>()
                .unwrap_or_else(|err| {
                    panic!(
                        "Couldn't parse as f64 in {}\nError: {}",
                        &path.as_ref().to_string_lossy(),
                        err
                    )
                })
        })
        .collect::<Vec<_>>();

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA_PATH: &'static str = "testdata/read_column_from_csv";
    #[test]
    fn read_first_column() {
        let actual = read_column_from_csv(format!("{}/sample.csv", TESTDATA_PATH), 0).unwrap();
        let expect = read_column_from_csv(format!("{}/column0.csv", TESTDATA_PATH), 0).unwrap();

        assert_eq!(actual, expect);
    }

    #[test]
    fn read_second_column() {
        let actual = read_column_from_csv(format!("{}/sample.csv", TESTDATA_PATH), 1).unwrap();
        let expect = read_column_from_csv(format!("{}/column1.csv", TESTDATA_PATH), 0).unwrap();

        assert_eq!(actual, expect);
    }

    #[test]
    #[should_panic]
    fn read_nonexistent_column() {
        read_column_from_csv(format!("{}/sample.csv", TESTDATA_PATH), 2).unwrap();
    }

    #[test]
    #[should_panic]
    fn read_csv_includes_invalid_value() {
        read_column_from_csv(format!("{}/invalid_value.csv", TESTDATA_PATH), 0).unwrap();
    }

    #[test]
    #[should_panic]
    fn read_nonexistent_file() {
        read_column_from_csv(format!("{}/nonexistent/file", TESTDATA_PATH), 0).unwrap();
    }
}
