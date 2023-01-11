use std::fmt::Display;
use array2d::Array2D;
use num_traits::Num;

pub fn delimiter<T: Num + Display>(array2d: Array2D<T>) -> impl Num + Display {
    if array2d.row_len() == 2 {
        // return Some(&array2d.get(0, 0));
        return 0;
    }
    2
}