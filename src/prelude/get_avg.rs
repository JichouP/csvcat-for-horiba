use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

pub fn get_avg_mut(mut vec: Vec<f64>, n: usize) -> Vec<f64> {
    if n == 0 {
        panic!("Error: Can't divide by 0")
    }
    // for i in 0..vec.len() {
    //     vec[i] /= n as f64;
    // }

    vec.par_iter_mut().for_each(|v| *v /= n as f64);

    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_avg_mut() {
        let actual = get_avg_mut(vec![1.0, 2.0], 2);
        let expect = vec![0.5, 1.0];
        assert_eq!(actual, expect);
    }

    #[test]
    #[should_panic]
    fn divided_by_0() {
        get_avg_mut(vec![], 0);
    }
}
