// * Print out the name and favorite colors of people aged 10 and under
use std::io;

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
}

impl Colors {
    fn new() -> Self {
        let mut buffer = String::new();
        loop {
            println!("color(red/green/blue/yellow/cyan/magenta): ");
            while io::stdin().read_line(&mut buffer).is_err() {
                println!("data error");
            }
            match buffer.trim() {
                "red" => return Self::Red,
                "green" => return Self::Green,
                "blue" => return Self::Blue,
                "yellow" => return Self::Yellow,
                "cyan" => return Self::Cyan,
                "magenta" => return Self::Magenta,
                _ => {
                    println!("invalid color.");
                    buffer.clear();
                    continue;
                }
            }
        }
    }

    fn print_color(&self) -> String {
        let color = match self {
            Self::Red => "red".to_owned(),
            Self::Green => "green".to_owned(),
            Self::Blue => "blue".to_owned(),
            Self::Yellow => "yellow".to_owned(),
            Self::Cyan => "Cyan".to_owned(),
            Self::Magenta => "Magenta".to_owned(),
        };
        color
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    color: Colors,
    age: u32,
}

impl Person {
    fn get_name() -> String {
        let mut buffer = String::new();
        loop {
            println!("name: ");
            while io::stdin().read_line(&mut buffer).is_err() {
                println!("data error");
            }
            match buffer.trim() {
                "" => {
                    println!("name cannot be empty");
                    buffer.clear();
                    continue;
                }
                name => return name.trim().to_owned(),
            }
        }
    }

    fn get_age() -> u32 {
        let mut buffer = String::new();
        loop {
            println!("age: ");
            while io::stdin().read_line(&mut buffer).is_err() {
                println!("data error");
            }
            match buffer.trim().parse() {
                Ok(num) => {
                    if num > 100 || num < 1 {
                        println!("age cannot be > 100 || < 1");
                        buffer.clear();
                        continue;
                    }
                    return num;
                }
                Err(_) => {
                    println!("only numbers between 1..100");
                    buffer.clear();
                    continue;
                }
            };
        }
    }

    fn new() -> Self {
        println!("_______________________");
        Self {
            name: Self::get_name(),
            age: Self::get_age(),
            color: Colors::new(),
        }
    }

    fn print_person(&self) {
        // * Print out the name and favorite colors of people aged 10 and under
        match self.age <= 10 {
            true => println!(
                "{} aged {} likes {}",
                self.name,
                self.age,
                self.color.print_color()
            ),
            false => (),
        }
    }
}

fn main() {
    let persons = vec![Person::new(), Person::new(), Person::new()];
    for person in &persons {
        person.print_person();
    }
}
