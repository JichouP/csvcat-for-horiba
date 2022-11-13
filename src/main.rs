use prelude::*;

mod prelude;

fn main() {
    #[cfg(test)]
    let csv_file_list = get_csv_file_list("testdata/ch").unwrap();
    #[cfg(not(test))]
    let csv_file_list = get_csv_file_list(".").unwrap();
    let groups = group_by_ch(csv_file_list).unwrap();
    let (sum, avg) = calc(groups);
    save(sum, avg);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn test() {
        let result = std::panic::catch_unwind(|| {
            main();
        });

        fs::remove_dir_all("aggregated").unwrap();

        result.unwrap();
    }
}
