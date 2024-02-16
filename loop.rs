fn looop(n: i32) {
    let mut i = 0; 
    loop {
        println!("count - {}", i);
        i += 1;
        if i == n {
            break;
        }
    }
}
fn main() {
    looop(5);
}