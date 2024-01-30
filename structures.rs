#![allow(dead_code)]

//Tuple Struct
struct Pair(i32, i32);

//C Struct
struct Person {
    name: String,
    age: u8
}

//Unit Struct
struct MyStruct;

trait MyTrait {
    fn my_method(&self) -> ();
}

impl MyTrait for MyStruct {
    fn my_method(&self) -> () {
        println!("MyStruct::my_method")
    }
}

fn main() {
    let name = String::from("Peter");
    println!("{}", name);

    let my_struct = MyStruct;
    my_struct.my_method();
}

/*
Structures Types
- Tuple 
- Unit
- C Structs

Unit - Often used as a placeholder when need to implement a trait

#![allow(dead_code)]
-> #![ ... ] syntax introduces an attribute, a special directive that modifies how the compiler handles the code.
-> allow(dead_code) specifically tells the compiler to suppress warnings related to "dead code."


*/