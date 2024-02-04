fn destroyer(var: String) {
    println!("Destrying - {}", var);
}

fn destroyer1(var: &mut String) {
    println!("{}", var);
    var.push('o');
    var.push_str(" dummy data.");
}

fn main() {
    let var = String::from("hello");
    destroyer(var);
    //println!("{}", var); -> compile time error as ownership has moved to the 
    //var of destroyer method

    let var1 = String::from("hello1");
    destroyer(var1.clone()); //pass copy of var1 to destroyer method
    println!("{}", var1); 

    let mut var2 = String::from("hell");
    destroyer1(&mut var2);// pass ref of var2
    println!("{}", var2);
}