fn main() {
    fn div(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("can not divide by zero.");
        }

        a / b
    }

    println!("divide - {}", div(10, 2));
    println!("divide - {}", div(10, 0));

}