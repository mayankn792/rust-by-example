use std::io;

fn main() {
    println!("Enter Input - ");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line.");
    
    println!("{num}");

}