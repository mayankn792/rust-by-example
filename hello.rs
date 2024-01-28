macro_rules! print_hello {
    () => {
        // The macro 'print_hello!' will expand into the contents of this block.
        println!("Hello.")
    }
}

fn main() {
    println!("Hello world!");
    print_hello!()
}



/*
println! -> macro that prints text on console

macro -> macros look like functions, except that their name ends with '!',
        but instead of generating a function call, macros are expanded 
        into source code that gets compiled with the rest of the program.
        
        * - Macros are created using 'macro_rules!' macro
        * - Rust provides a powerful macro system that allow metaprogramming

metaprogramming -> Metaprogramming is a programming technique where a program 
        manipulates other programs as its data.
        
*/