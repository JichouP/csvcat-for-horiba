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

#[cfg(test)]
mod tests {
    use super::*;
    const CSV_PATH: &'static str = "testdata/sample.csv";
    #[test]
    fn read_first_column() {
        let actual = read_column_from_csv(CSV_PATH, 0).unwrap();
        let expect = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

        assert_eq!(actual, expect);
    }

    #[test]
    fn read_second_column() {
        let actual = read_column_from_csv(CSV_PATH, 1).unwrap();
        let expect = vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0, 20.0];

        assert_eq!(actual, expect);
    }

    #[test]
    #[should_panic]
    fn read_nonexistent_column() {
        read_column_from_csv(CSV_PATH, 2).unwrap();
    }

    #[test]
    #[should_panic]
    fn read_nonexistent_file() {
        read_column_from_csv("nonexistent/file", 0).unwrap();
    }
}
