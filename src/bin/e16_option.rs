// Topic: Optins
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    locker_no: Option<u32>,
}

impl Student {
    fn create_student(name: &str, locker_no: Option<u32>) -> Self {
        match locker_no {
            Some(num) => {
                return Self {
                    name: name.to_owned(),
                    locker_no: Some(num),
                }
            }
            None => {
                return Self {
                    name: name.to_owned(),
                    locker_no: None,
                }
            }
        }
    }
    fn locker_choice() -> Option<u32> {
        let choice = get_true_false();
        match choice {
            true => return Some(get_locker_no()),
            false => return None,
        }
    }
}

fn get_true_false() -> bool {
    let mut input = String::new();
    loop {
        input.clear();
        println!("Does the student have a locker no(yes/no): ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read_line");
        let input: bool = match input.to_lowercase().trim() {
            "yes" => true,
            "no" => false,
            _ => {
                println!("only yes/no allowed");
                continue;
            }
        };
        return input;
    }
}

fn get_name() -> String {
    let mut input = String::new();
    loop {
        input.clear();
        println!("Name of the student: ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read_line");
        let input: String = match input.to_lowercase().trim() {
            "" => {
                println!("empty names not allowed");
                continue;
            }
            name_input => name_input.to_owned(),
        };
        return input;
    }
}

fn get_locker_no() -> u32 {
    let mut value = String::new();
    println!("input an number between 1 and 100: ");
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
                    println!("no more than 100");
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

fn print_student(student: &Student) {
    match student.locker_no {
        Some(num) => println!("student: {} with locker {}", student.name, num),
        None => println!("student: {} has a general locker", student.name),
    }
}

fn main() {
    let new_student1 = Student::create_student(&get_name(), Student::locker_choice());
    let new_student2 = Student::create_student(&get_name(), Student::locker_choice());
    print_student(&new_student1);
    print_student(&new_student2);
}
