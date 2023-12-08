#![allow(dead_code, unused_variables, unused_imports)]

use std::io;

// * Use an enum to represent all types of employees
#[derive(Debug)]
enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

#[derive(Debug)]
enum Status {
    Active,
    Terminated,
}

// * Use a struct to store the employee type and whether they are still employed
#[derive(Debug)]
struct Employee {
    name: String,
    position: Position,
    status: Status,
}

impl Employee {
    fn get_name() -> String {
        let mut input = String::new();
        loop {
            input.clear();
            println!("employee's name: ");
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line.");
            match input.trim() {
                "" => {
                    println!("err: empty name input.");
                    continue;
                }
                name => return name.to_owned(),
            }
        }
    }
    // * Use a function that returns a Result to determine if the employee may enter the building
    fn try_access(&self) -> Result<(), String> {
        match self.status {
            Status::Terminated => return Err("terminated".to_owned()),
            _ => (),
        }
        match self.position {
            Position::Maintenance => Ok(()),
            Position::Manager => Ok(()),
            Position::Marketing => Ok(()),
            _ => Err("invalid position".to_owned()),
        }
    }

    // * Print whether the employee may access the building
    // * Must use a function that utilizes the question mark operator to do this
    fn print_access(&self) -> Result<(), String> {
        let atempt_access = Self::try_access(self)?;
        println!("access ok");
        Ok(())
    }

    fn get_employment_status() -> Status {
        let mut input = String::new();
        loop {
            input.clear();
            println!("employed(y/n): ");
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line.");
            match input.to_lowercase().trim() {
                "y" => {
                    return Status::Active;
                }
                "n" => return Status::Terminated,
                "" => {
                    println!("empty values not allowed. (y/n)");
                    continue;
                }
                _ => {
                    println!("only (y/n) allowed.");
                    continue;
                }
            }
        }
    }

    fn get_employment_position() -> Position {
        println!("employee type possible values:");
        let mut input = String::new();
        loop {
            input.clear();
            println!("mang/markt/maintn/spv/staff/tech");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line.");
            match input.to_lowercase().trim() {
                "mang" => return Position::Manager,
                "markt" => return Position::Marketing,
                "maintn" => return Position::Maintenance,
                "staff" => return Position::KitchenStaff,
                "spv" => return Position::LineSupervisor,
                "tech" => return Position::AssemblyTechnician,
                "" => {
                    println!("empty values not allowed.");
                    continue;
                }
                _ => continue,
            }
        }
    }

    fn create_employee() -> Self {
        Self {
            name: Self::get_name(),
            position: Self::get_employment_position(),
            status: Self::get_employment_status(),
        }
    }
}

fn get_num() -> u32 {
    println!("num of employees: ");
    let mut input = String::new();
    loop {
        input.clear();
        println!("(only positive integers between 1..5): ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        match input.trim().parse() {
            Ok(num) => {
                if num < 1 {
                    println!("no employees cannot be < 1");
                    continue;
                } else if num > 5 {
                    println!("no employees cannot be > 5");
                    continue;
                } else {
                    return num;
                }
            }
            Err(_) => {
                println!("only numbers between 1..5 allowed!");
                continue;
            }
        };
    }
}

fn empl_database(num: u32) -> Vec<Employee> {
    let mut database: Vec<Employee> = vec![];
    for i in 0..num {
        println!("employee no {}", i + 1);
        database.push(Employee::create_employee());
    }
    database
}

fn print_database(vector: &Vec<Employee>) {
    for i in vector {
        println!("{:?}", i);
        println!("{:?} has acces: {:?}", i.name, i.print_access());
    }
}

fn main() {
    let num_employees = get_num();
    let my_empl_database = empl_database(num_employees);
    print_database(&my_empl_database);
}
