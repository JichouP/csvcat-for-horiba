use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub fn get_sum_mut(mut vec1: Vec<f64>, vec2: &Vec<f64>) -> Vec<f64> {
    let len = vec1.len();
    if len != vec2.len() {
        panic!("CSV lengths do not match")
    }

    vec1.par_iter_mut().zip(vec2).for_each(|(v1, v2)| *v1 += v2);

    vec1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_sum_mut() {
        let actual = get_sum_mut(vec![1.0, 2.0], &vec![3.0, 4.0]);
        let expect = vec![4.0, 6.0];

        assert_eq!(actual, expect);
    }

    #[test]
    #[should_panic]
    fn different_len() {
        get_sum_mut(vec![1.0, 2.0], &vec![3.0]);
    }
}
