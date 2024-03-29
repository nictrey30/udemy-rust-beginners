// Topic: Implementing functionality with the impl keyword
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

#![allow(dead_code, unused_variables, unused_imports)]

use std::io;

#[derive(Debug)]
enum Colors {
    Black,
    Grey,
    Yellow,
    White,
}

impl Colors {
    fn color_string(&self) -> String {
        match &self {
            Colors::Black => "black".to_string(),
            Colors::Grey => "grey".to_string(),
            Colors::Yellow => "yellow".to_string(),
            Colors::White => "white".to_string(),
        }
    }
}

#[derive(Debug)]
struct Dimensions {
    width: f64,
    length: f64,
    height: f64,
}

impl Dimensions {
    fn return_dimension(width: f64, length: f64, height: f64) -> Self {
        Self {
            width,
            length,
            height,
        }
    }
}

#[derive(Debug)]
struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Colors,
}

impl Box {
    fn create_box(dimensions: Dimensions, color: Colors, weight: f64) -> Self {
        Self {
            dimensions,
            color,
            weight,
        }
    }
    fn print_box(&self) {
        println!("box: {:?}", &self);
    }
}

fn get_dimension() -> f64 {
    let mut value = String::new();
    loop {
        value.clear();
        println!("input a number value: ");
        io::stdin()
            .read_line(&mut value)
            .expect("failed to read line.");
        let value: f64 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("only numbers allowed");
                continue;
            }
        };
        println!("________________________");
        return value.abs();
    }
}

fn display_available_colors() {
    println!("The available colors are: ");
    for i in [Colors::Black, Colors::Grey, Colors::White, Colors::Yellow] {
        println!("{}", i.color_string());
    }
    println!("________________________");
}

fn get_color() -> Colors {
    display_available_colors();
    let mut color = String::new();
    loop {
        println!("please choose a valid color");
        color.clear();
        io::stdin()
            .read_line(&mut color)
            .expect("failed to read line");
        match color.to_lowercase().trim() {
            "black" => return Colors::Black,
            "white" => return Colors::White,
            "yellow" => return Colors::Yellow,
            "grey" => return Colors::Grey,
            _ => {
                println!("only valid colors allowed");
                continue;
            }
        };
    }
}

fn get_dimension_type(dimension_type: &str) -> f64 {
    println!("input dimensions -> {}:", dimension_type);
    get_dimension()
}

fn main() {
    println!("Create a box:");
    let width = get_dimension_type("width");
    let length = get_dimension_type("length");
    let height = get_dimension_type("height");
    let weight = get_dimension_type("weight");
    let color: Colors = get_color();

    let my_box: Box = Box::create_box(
        Dimensions::return_dimension(width, length, height),
        color,
        weight,
    );
    my_box.print_box();
}
