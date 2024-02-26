// Topic: Implementing functionality with the impl keyword
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color

// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
use std::io;

#[derive(Debug)]
enum BoxColors {
    Grey,
    Brown,
    White,
    Yellow,
}

impl BoxColors {
    fn new() -> Self {
        let mut buffer = String::new();
        println!("box color(grey, brown, white or yellow): ");
        loop {
            while io::stdin().read_line(&mut buffer).is_err() {
                println!("data error");
            }
            match buffer.trim() {
                "grey" => return Self::Grey,
                "brown" => return Self::Brown,
                "white" => return Self::White,
                "yellow" => return Self::Yellow,
                _ => {
                    println!("invalid color. choose (grey, brown, white or yellow): ");
                    buffer.clear();
                    continue;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Dimensions {
    height: f64,
    length: f64,
    width: f64,
}

impl Dimensions {
    fn return_dimensions() -> Self {
        Self {
            height: get_dimension("height"),
            length: get_dimension("length"),
            width: get_dimension("width"),
        }
    }
}

#[derive(Debug)]
struct ShippingBox {
    dimensions: Dimensions,
    color: BoxColors,
    weight: f64,
}

impl ShippingBox {
    fn new(dimensions: Dimensions, color: BoxColors) -> Self {
        Self {
            dimensions,
            color,
            weight: get_dimension("weight"),
        }
    }

    fn print_box(&self) {
        let volume = self.dimensions.height * self.dimensions.width * self.dimensions.length;
        let volume_in_liters = volume as f64 / 1000.0;

        println!(
            "box with H{}xL{}xW{} cm, volume {} L and weight {} Kg / color {:?}",
            self.dimensions.height,
            self.dimensions.length,
            self.dimensions.width,
            volume_in_liters,
            self.weight,
            self.color
        );
    }
}

fn get_dimension(dimension_type: &str) -> f64 {
    let mut buffer = String::new();
    loop {
        println!("enter box {dimension_type}:");
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data error");
        }
        match buffer.trim().parse() {
            Ok(num) => {
                if num <= 0.0 {
                    println!("only positive integers allowed.");
                    buffer.clear();
                    continue;
                }
                return num;
            }
            Err(_) => {
                println!("only numbers allowed");
                buffer.clear();
                continue;
            }
        }
    }
}

fn main() {
    let my_box = ShippingBox::new(Dimensions::return_dimensions(), BoxColors::new());
    my_box.print_box();
}
