fn main() {
    let s = String::from("Hello");
    println!("{}", s);

    s.push('c');
    println!("{}", s);

    s.push_str(" this is string");
    println!("{}", s);
    
    let second_char = s.chars().nth(2)
    println!("{}", second_char)

}