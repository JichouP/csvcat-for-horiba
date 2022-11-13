use prelude::*;
use std::env;

mod prelude;

fn main() {
    let current_dir = env::current_dir().expect("Error: Can't get current directory");
    let csv_file_list = get_csv_file_list(current_dir).unwrap_or_else(|err| panic!("{}", err));
    let groups = group_by_ch(csv_file_list).unwrap_or_else(|err| panic!("{}", err));
    let (sum, avg) = calc(groups);
}
