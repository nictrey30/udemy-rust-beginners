// Program requirements:
// * Prints the name of a color to the terminal
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color name to print

use std::io;

enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
}

fn get_input() -> String {
    let mut buffer = String::new();
    println!("please input a color: ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data input error, try again!");
        }
        // checking if the input has 0 length
        if buffer.trim().chars().count() == 0 {
            println!("please input an non-empty value");
            buffer.clear();
            continue;
        }
        return buffer.trim().to_lowercase().to_owned();
    }
}

fn print_color(data: &Colors) {
    match data {
        Colors::Red => println!("color red"),
        Colors::Green => println!("color green"),
        Colors::Blue => println!("color blue"),
        Colors::Yellow => println!("color yellow"),
        Colors::Magenta => println!("color magenta"),
        Colors::Cyan => println!("color cyan"),
    }
}

fn check_color(input: &str) -> Result<Colors, String> {
    match input.trim() {
        "red" => Ok(Colors::Red),
        "green" => Ok(Colors::Green),
        "blue" => Ok(Colors::Blue),
        "yellow" => Ok(Colors::Yellow),
        "cyan" => Ok(Colors::Cyan),
        "magenta" => Ok(Colors::Magenta),
        _ => Err("color not found".to_owned()),
    }
}

fn main() {
    let my_color = check_color(&get_input());
    match my_color {
        Ok(color) => print_color(&color),
        Err(e) => println!("{:?}", e),
    }
}
