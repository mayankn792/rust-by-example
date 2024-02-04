struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "sheeeeeeepp"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "cooooooow"
    }
}

fn pick_random() -> Box<dyn Animal> {
    let random_number: f64 = 0.7;
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let animal = pick_random();
    println!("you pick - {}", animal.noise());
}