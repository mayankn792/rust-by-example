struct GenVal<T> {
    val: T,
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

fn main() {
    let y = GenVal { val: 7 };
    let x = GenVal { val: 3i32 };
    let z = GenVal { val: 32i64 };
    println!("x - {} | y - {} | z - {}", x.value(), y.value(), z.value());
}