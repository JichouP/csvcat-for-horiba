use std::{io, path::PathBuf};

pub fn group_by_ch(paths: Vec<PathBuf>) -> io::Result<Vec<Vec<PathBuf>>> {
    let max_ch_number = get_max_ch(&paths);

    let result: Vec<Vec<PathBuf>> = (1..max_ch_number + 1)
        .map(|ch| {
            paths
                .iter()
                .cloned()
                .filter(|path| is_contain_target_ch(path, ch))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(result)
}

fn get_max_ch(paths: &Vec<PathBuf>) -> usize {
    paths
        .into_iter()
        .map(|path| {
            let path = path.to_string_lossy().to_string();
            path.split("CH")
                .last()
                .expect(r#"Error: Can't split by "CH""#)
                .split('.')
                .next()
                .expect("Error: Can't split by '.'")
                .parse::<usize>()
                .expect("Error: Can't parse the CH number")
        })
        .max()
        .expect("Error: Can't find CSV file")
}

fn is_contain_target_ch(path: &PathBuf, ch: usize) -> bool {
    path.to_str()
        .expect("Error: Can't convert path to &str")
        .contains(&format!("-CH{}.", ch))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    const TESTDATA_PATH: &'static str = "testdata/ch";

    #[test]
    fn should_group_by_ch() {
        let paths = get_csv_file_list(TESTDATA_PATH).unwrap();
        let actual = group_by_ch(paths).unwrap();

        assert_eq!(actual.len(), 4);
        assert_eq!(actual[0].len(), 2);
        assert_eq!(actual[1].len(), 2);
        assert_eq!(actual[2].len(), 2);
        assert_eq!(actual[3].len(), 2);
    }
}
