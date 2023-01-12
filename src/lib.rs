use std::fmt::Display;
use array2d::Array2D;
use num_traits::{Num, NumAssignOps, Zero};

pub fn delimiter<T: NumAssignOps + Num + Display + Copy + Zero>(array2d: &Array2D<T>) -> T {
    let n = array2d.num_rows();
    return if n == 2 {
        array2d.get(0, 0).copied().unwrap() * array2d.get(1, 1).copied().unwrap()
            - array2d.get(0, 1).copied().unwrap() * array2d.get(1, 0).copied().unwrap()
    } else {
        let mut result: T = Zero::zero();
        let mut arrays: Vec<Array2D<T>> = Vec::with_capacity(n);
        let rows_before: Vec<Vec<T>> = array2d.rows_iter()
            .map(|v| v.map(|x| *x).collect())
            .collect();
        let mut rows_after: Vec<Vec<Vec<T>>> = Vec::with_capacity(n);
        for _ in 0..rows_after.capacity() {
            rows_after.push(rows_before.clone());
        }
        for i in 0..rows_after.capacity() {
            rows_after[i].remove(0);
        }
        for i in 0..rows_after.capacity() {
            for j in 0..(rows_after[i].capacity() - 1) {
                rows_after[i][j].remove(i);
            }
        }
        for i in 0..rows_after.capacity() {
            arrays.push(Array2D::from_rows(&rows_after[i]).unwrap());
        }
        for i in 0..n {
            if (1 + i) % 2 == 0 {
                result -= rows_before[0][i] * delimiter(&arrays[i]);
            } else {
                result += rows_before[0][i] * delimiter(&arrays[i]);
            }
        }
        result
    }
}