fn is_even(n: u32) -> bool {
    n % 2 == 0
}
fn main() {

    let upper = 1000;
    let sum: u32 = (0..=upper)
                    .map(|n| n * n)
                    .filter(|&n| is_even(n))
                    .sum();

    println!("{}", sum);
}