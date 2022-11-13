use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::fs;

pub fn save(sum: Vec<Vec<f64>>, avg: Vec<Vec<f64>>) {
    fs::create_dir_all("aggregated").expect("Error: Can't create dist dir");

    sum.par_iter().enumerate().for_each(|(i, v)| {
        fs::write(
            format!("aggregated/sum-CH{}.csv", i + 1),
            v.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .unwrap_or_else(|err| panic!("{}", err));
    });

    avg.par_iter().enumerate().for_each(|(i, v)| {
        fs::write(
            format!("aggregated/avg-CH{}.csv", i + 1),
            v.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .unwrap_or_else(|err| panic!("{}", err));
    });
}
