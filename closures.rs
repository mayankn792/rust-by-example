fn main() {
    let var = 100;
    let closure = |i: i32| -> i32 {i + var};
    let n = || 1; // return 1 type i32
    println!("{}", closure(n()));
}