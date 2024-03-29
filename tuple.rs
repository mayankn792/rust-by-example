// Activity - 2.2
use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn main() {
    let mat = Matrix(1.1, 1.2, 1.3, 1.4);
    println!("{}", mat);
}
