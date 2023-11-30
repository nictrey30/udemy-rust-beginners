// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
use std::io;

#[derive(Debug)]
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

impl Colors {
    fn color_string(&self) -> String {
        match &self {
            Colors::Black => return "black".to_string(),
            Colors::Blue => return "blue".to_string(),
            Colors::Green => return "green".to_string(),
            Colors::Red => return "red".to_string(),
            Colors::Cyan => return "cyan".to_string(),
            Colors::Magenta => return "magenta".to_string(),
            Colors::Yellow => return "yellow".to_string(),
            Colors::White => return "white".to_string(),
        }
    }
}

struct Person {
    name: String,
    age: u32,
    fav_color: Colors,
}

impl Person {
    fn get_name() -> String {
        let mut value = String::new();
        loop {
            value.clear();
            println!("name: ");
            io::stdin()
                .read_line(&mut value)
                .expect("failed to read line");
            if value.trim() == "" {
                println!("empty values not allowed.");
                continue;
            }
            return value.trim().to_owned();
        }
    }

    fn get_age() -> u32 {
        let mut value = String::new();
        println!("input an age between 1 and 100: ");
        loop {
            value.clear();
            io::stdin()
                .read_line(&mut value)
                .expect("failed to read line.");
            let value: u32 = match value.trim().parse() {
                Ok(num) => {
                    if num == 0 {
                        println!("0 not allowed");
                        continue;
                    } else if num > 100 {
                        println!("no more than 10");
                        continue;
                    } else {
                        num
                    }
                }
                Err(_) => {
                    println!("only numbers between 1..100 allowed.");
                    continue;
                }
            };
            return value;
        }
    }

    fn display_available_colors() {
        for i in [
            Colors::Black,
            Colors::Blue,
            Colors::Green,
            Colors::Red,
            Colors::Cyan,
            Colors::Magenta,
            Colors::White,
            Colors::Yellow,
        ] {
            println!("{}", i.color_string());
        }
        println!("_________________________");
    }

    fn get_color() -> Colors {
        let mut color = String::new();
        loop {
            println!("please choose a valid color from: ");
            Self::display_available_colors();
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

    fn create_person() -> Self {
        Self {
            name: Self::get_name(),
            age: Self::get_age(),
            fav_color: Self::get_color(),
        }
    }

    fn display_person(&self, info: &str) {
        match info {
            "name" => {
                println!("name: {}", self.name);
            }
            "age" => {
                println!("age: {}", self.age);
            }
            "color" => {
                println!("{:?}", self.fav_color);
            }
            _ => {
                println!("error");
            }
        }
    }
}

fn display_option() -> String {
    let mut value = String::new();
    loop {
        value.clear();
        println!("choose field: name/ age/ color:");
        io::stdin()
            .read_line(&mut value)
            .expect("failed to read line");
        match value.to_lowercase().trim() {
            "name" => return "name".to_owned(),
            "age" => return "age".to_owned(),
            "color" => return "color".to_owned(),
            _ => {
                println!("Only -> name/ age/ color <- allowed.");
                continue;
            }
        };
    }
}

fn main() {
    let person = Person::create_person();
    let display_option = display_option();
    Person::display_person(&person, &display_option[..]);
}
