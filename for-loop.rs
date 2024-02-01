fn main() {
    let start = 1;
    let end = 10;
    for i in start .. end {
        println!("{}", i);
    }

    let names = vec!["Bob", "John", "Wick"];
    for name in names.iter() {
        match name {
            &"Wick" => println!("John wick."),
            _ => println!("Not found."),
        }
    }

    println!("{:?}", names);
}