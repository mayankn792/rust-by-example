//Its size matches the size of a pointer on your system, typically 32 bits (4 bytes)
// on 32-bit architectures and 64 bits (8 bytes) on 64-bit architectures.
fn list_length<T>(list: &[T]) -> usize {
    list.len()
}

fn main() {
    let str_list = vec!["hello", "world"];
    let int_list = vec![1, 2, 3];

    let str_list_len = list_length(&str_list);
    let int_list_len = list_length(&int_list);

    println!("str_list_len {} | int_list_len {}", str_list_len, int_list_len);
}