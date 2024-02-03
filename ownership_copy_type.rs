#[allow(dead_code)]
//Primitive types in Rust are copy types. This means that 
//when you pass them to a function, they are copied, not moved
//ownership rule apply to non copy var
fn destroyer(mut n: u32) {
    n = n + 1;
    println!("destroying {}", n);
}

fn main() {
    let x = 5u32;
    let y = x; // x and y both have independent copies of 5
    destroyer(x);
    println!("{}, {}", x, y);
}