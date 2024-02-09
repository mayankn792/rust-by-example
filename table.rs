use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number - ");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {println!("Read - {}", input);}
        Err(e) => println!("Error : {}", e),
    }
    
    let n = input.trim_end().parse::<usize>().unwrap();
    for i in 1 ..= 10 {
        println!("{} * {} = {}", n, i, n * i);
    }
}