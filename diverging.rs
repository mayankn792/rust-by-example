#[allow(unreachable_code)]

fn main() {
    fn print(n: u32) -> ! {
        println!("{}", n);
        panic!("halted.")
    }

    print(10);

    //unreachable code
    println!("waiting print to complete.")
}
