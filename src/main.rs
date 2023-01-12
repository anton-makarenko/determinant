use array2d::Array2D;
use determinant::delimiter;

fn main() {
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6], vec![4, 5, 6]];
    println!("{}", delimiter(Array2D::from_rows(&rows).unwrap()));
}