use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn get_csv_file_list<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let file_list: Vec<PathBuf> = fs::read_dir(path)?
        .into_iter()
        .filter_map(|entry| {
            let path = entry.expect("Failed to exec read_dir").path();

            let ext = path.extension().and_then(|ext| ext.to_str());

            match ext {
                Some("csv") => Some(path),
                _ => None,
            }
        })
        .collect();

    Ok(file_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA_PATH: &'static str = "testdata/get_csv_file_list";
    #[test]
    fn should_return_only_csv_files() {
        let actual = get_csv_file_list(TESTDATA_PATH).unwrap();
        let expect = vec![PathBuf::from(format!("{}/dummy.csv", TESTDATA_PATH))];

        assert_eq!(actual, expect);
    }

    #[test]
    #[should_panic]
    fn read_nonexistence_dir() {
        get_csv_file_list("nonexistence_dir").unwrap();
    }
}
