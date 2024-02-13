fn rec(n : u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * rec(n - 1)
    }
}
fn main() {
    println!("{}", rec(5));
}