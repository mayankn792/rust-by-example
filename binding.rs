fn age() -> u32 {
    15
}

fn main() {
    println!("{}", age());
    match age() {
        0 => println!("hey! zero year old."),
        n @ 1 ..= 12 => println!("You are a child of age {}", n),
        n @ 13 ..= 19 => println!("You are a teen of age {}", n),
        _ => println!("To old to play this game."),
    }
}