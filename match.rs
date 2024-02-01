fn main() {
    let number = 13;
    match number {
        1 => println!("This is one"),
        2 | 3 | 4 | 13 => println!("2 | 3 | 4"),
        13 ..=19 => println!("13 ..=19"),
        _ => println!("catch - all arm")
    }
}