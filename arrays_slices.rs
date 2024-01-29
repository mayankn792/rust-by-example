fn slice(s: &[i32]) {
    println!("{}", s.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 20] = [0; 20];

    // Borrow the whole array
    slice(&xs);

    // Borrow a section of array
    slice(&ys[1 .. 4]);

    //Accessing elements
    for i in 0 .. xs.len() + 3 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Out of range.")
        }
    }
}