use std::io;
use std::num::IntErrorKind::Empty;

#[derive(Debug)]
struct Student {
    name: String,
    locker: Option<i32>,
}

impl Student {
    fn new() -> Self {
        Self {
            name: get_name(),
            locker: get_locker_num(),
        }
    }

    fn print_student(&self) {
        println!("student's name: {:?}", self.name);
        match self.locker {
            Some(num) => println!("locker no: {:?}", num),
            None => println!("{:?} doesn't have a locker assigned", self.name),
        }
    }
}

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

fn get_locker_num() -> Option<i32> {
    let mut buffer = String::new();
    println!("input an number (0..100) or nothing:");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data error");
        }
        match buffer.trim().parse() {
            Ok(num) => {
                if num < 0 || num > 100 {
                    println!("number range error(0..100)");
                    buffer.clear();
                    continue;
                }
                return Some(num);
            }
            Err(e) => {
                if e.kind() == &Empty {
                    return None;
                }
                println!("only numbers between 1..100 allowed.");
                buffer.clear();
                continue;
            }
        };
    }
}

fn main() {
    let new_student = Student::new();
    new_student.print_student();
}
