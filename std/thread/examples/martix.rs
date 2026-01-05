use std::thread::Result;

use thread::Matrix;
fn main() -> Result<()> {
    let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
    let b = Matrix::new(3, 2, vec![7, 8, 9, 10, 11, 12]);
    println!("{:?}", a * b);
    Ok(())
}
