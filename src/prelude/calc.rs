use super::{get_sum_mut, read_column_from_csv};
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::path::PathBuf;

pub fn calc(groups: Vec<Vec<PathBuf>>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let csv_len = read_column_from_csv(groups[0][0].clone(), 0)
        .expect("Error: Can't load csv file")
        .len();

    let sum: Vec<Vec<f64>> = groups
        .into_par_iter()
        .map(|group| {
            group
                .par_iter()
                .map(|path| read_column_from_csv(path, 0).unwrap_or_else(|err| panic!("{}", err)))
                .reduce(|| vec![0.0; csv_len], |a, b| get_sum_mut(a, &b))
        })
        .collect();

    let avg: Vec<Vec<f64>> = sum
        .par_iter()
        .map(|v| v.into_par_iter().map(|v| v / csv_len as f64).collect())
        .collect();

    (sum, avg)
}
