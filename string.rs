fn main() {
    let mut s = String::from("Hello");
    println!("{}", s);

    s.push('c');
    println!("{}", s);

    s.push_str(" this is string");
    println!("{}", s);
}