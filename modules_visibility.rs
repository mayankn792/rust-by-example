//An attribute is metadata applied to some module, crate or item
#[allow(dead_code)]

mod print_mod {
    //By default, the items in a module have private visibility
    fn private_mod() {
        println!("can not access me from outside of this module.");
    }

    pub fn public_mod() {
        println!("public module.")
    }
}
fn main() {
    //print_mod::private_mod(); -> can not access
    print_mod::public_mod();
}