trait Fall {
    fn hit_ground(&self);
}

struct Vase;
impl Fall for Vase {
    fn hit_ground(&self) {
        println!("The vase broke");
    }
}
struct Cat;
impl Fall for Cat {
    fn hit_ground(&self) {
        println!("The cat casually walked away");
    }
}

// use impl Trait as a function argument to pass data via Trait
fn fall(thing: impl Fall) {
    thing.hit_ground();
}

fn main() {
    fall(Vase {});
    fall(Cat {});
}
