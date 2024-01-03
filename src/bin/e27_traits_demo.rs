#![allow(dead_code, unused_variables, unused_imports)]

// Traits - a way to specify that some functionality exists
// - used to standardize functionality across multiple diffrent types.
// Standardization permits functions to operate on multiple diffrent types

// Recap:
// Traits define similar functionality for diffrent types
// Trait functions are just regular functions(can accept arguments and return values)
// use impl Trait as a function argument to pass data via Trait
trait Noise {
    fn make_noise(&self);
}

struct Person;
impl Noise for Person {
    fn make_noise(&self) {
        println!("hello");
    }
}

struct Dog;
impl Noise for Dog {
    fn make_noise(&self) {
        println!("woof");
    }
}

// noisy: impl Noise => is read as: noisy type is something that implements the Noise Trait
fn hello(noisy: impl Noise) {
    noisy.make_noise();
}

fn main() {
    hello(Person {});
    hello(Dog {});
}
