fn create() {
    let _box = Box::new(32i64);
}

struct ToDrop;
impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("Droping drop.");
    }
}

fn main() {
    for _ in 0 .. 10 {
        create();
    } 

    let _x = ToDrop;
    println!("create drop");
}