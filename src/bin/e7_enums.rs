// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
use std::io;
// import the macros needed
use strum_macros::{EnumString, EnumVariantNames};
// You need to import the trait, to have access to VARIANTS
use strum::VariantNames;

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab-case")]
enum Colors {
    Black,
    Blue,
    Green,
    Red,
    Cyan,
    Magenta,
    Yellow,
    White,
}

fn display_available_colors() {
    println!("The available colors are: ");
    for i in Colors::VARIANTS {
        println!("{}", i.to_lowercase());
    }
}

fn input_color() -> Colors {
    display_available_colors();
    let mut color = String::new();
    loop {
        println!("please choose a valid color");
        io::stdin()
            .read_line(&mut color)
            .expect("failed to read line");
        match color.to_lowercase().trim() {
            "black" => return Colors::Black,
            "blue" => return Colors::Blue,
            "green" => return Colors::Green,
            "red" => return Colors::Red,
            "cyan" => return Colors::Cyan,
            "magenta" => return Colors::Magenta,
            "yellow" => return Colors::Yellow,
            "white" => return Colors::White,
            _ => {
                println!("only valid colors allowed");
                color.clear();
                continue;
            }
        };
    }
}

fn print_color(color: &Colors) {
    println!("The color you chose was {:?}", color);
}

fn main() {
    assert_eq!(
        ["black", "blue", "green", "red", "cyan", "magenta", "yellow", "white"],
        Colors::VARIANTS
    );
    print_color(&input_color());
}
