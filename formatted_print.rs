fn main() {
    // {} -> Placeholder for Parameters
    println!("Value of x - {}", 12);

    //Positional Parameters
    println!("Second argument - {1} | First argument - {0}", 1, 2);

    //Named Parameters
    println!("{first} {second} {third}", 
                third="3rd",
                first="1st",
                second="2nd");
    
    eprint!("io::stderr");
    println!();
    println!("{Pi}", Pi=3.142);

    let x = format!("Format {}", 10);
    println!("{}", x)

}

/*
Formatting -> Printing in rust is handled by a series of macros defined in std::fmt 
        some of which include
        * - format! - write formatted text to String
        * - println! - print text to the console (io::stdout)
        * - eprintln! - printed to the standard error (io::stderr)

        -> Only types that implement fmt::Display can be formatted with `{}`.
        -> User defined types do not implement fmt::Display by default.

std::fmt - Utilities for formatting and printing strings
std::format - create a string using interpolation of runtime expressions.


*/