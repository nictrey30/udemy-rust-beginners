// borrow example
#[derive(Debug, PartialEq)]
enum Light {
    Bright,
    Dull,
}

// we are borrowing/referencing the data, and the display_light is not allowed to delete the data because it is not the owner
fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

// any function that owns data is required to delete the data once the function completes
fn switch_light(light: Light) -> Light {
    if light == Light::Bright {
        Light::Dull
    } else {
        Light::Bright
    }
}

fn main() {
    let mut dull = Light::Dull;
    display_light(&dull);
    dull = Light::Bright;
    display_light(&dull);
    println!("the current {:?}", dull);
    let bright = switch_light(dull);
    println!("is now {:?}", bright);
}
