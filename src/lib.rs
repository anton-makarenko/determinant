use std::fmt::Display;
use array2d::Array2D;
use num_traits::Num;

pub fn delimiter<T: Num + Display + Copy>(array2d: Array2D<T>) -> impl Num + Display + Copy {
    let n = array2d.num_rows();
    if n == 2 {
        return array2d.get(0, 0).copied().unwrap() * array2d.get(1, 1).copied().unwrap()
            - array2d.get(0, 1).copied().unwrap() * array2d.get(1, 0).copied().unwrap();
    }
    else {
        let arrays: Vec<Array2D<T>> = Vec::with_capacity(n);
        // let mut rows = Vec::with_capacity(n);
        // let rows_iter = array2d.rows_iter();
        let rows_before: Vec<Vec<T>> = array2d.rows_iter()
            .map(|v| v.map(|x| *x).collect())
            .collect();
        let mut rows_after: Vec<Vec<Vec<T>>> = Vec::with_capacity(n);
        for i in 0..rows_after.capacity() {
            rows_after[i] = Vec::with_capacity(n - 1);
            for j in 0..rows_after[i].capacity() {
                rows_after[i][j] = Vec::with_capacity(n - 1);
            }
        }
        // for ri in rows_iter {
        //     let row: Vec<T> = ri.map(|x| *x).collect();
        //     rows.push(row);
        // }
        array2d.get(0, 0).copied().unwrap()
    }
}