use array2d::Array2D;
use determinant::delimiter;

fn main() {
    println!("{}", delimiter(Array2D::filled_with(0, 2, 2)));
}