#[derive(Debug)]
struct Print(i32);

#[derive(Debug)]
#[allow(dead_code)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    println!("{:?}", 12);
    println!("{:?}", Print(3));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}

/*
struct Print(i32); -> This structure cannot be printed either with fmt::Display
            or fmt::Debug


#[derive(Debug)]
struct Print(i32); -> This makes structure printable with fmt::Debug. The Problem 
            with this is there is no control over how result looks.

{:#?} - pretty printing

*/