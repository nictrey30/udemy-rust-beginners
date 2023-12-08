// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
#![allow(dead_code, unused_variables, unused_imports)]

use std::io;

#[derive(Debug)]
enum Access {
    Maintenance(bool),
    Marketing(bool),
    Manager(bool),
    LineSupervisor(bool),
    KitchenStaff(bool),
    AssemblyTechnician(bool),
}

#[derive(Debug)]
struct Employee {
    name: String,
    employee_type: Access,
    employed_status: bool,
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

    fn get_access_type(access: &str) -> Result<Access, String> {
        match access.to_lowercase().trim() {
            "mang" => return Ok(Access::Manager(true)),
            "markt" => return Ok(Access::Marketing(true)),
            "maintn" => return Ok(Access::Maintenance(true)),
            "staff" => return Ok(Access::KitchenStaff(false)),
            "spv" => return Ok(Access::LineSupervisor(false)),
            "tech" => return Ok(Access::AssemblyTechnician(false)),
            "" => return Err("empty epmloyee type input.".to_owned()),
            _ => return Err("employee type not found.".to_owned()),
        }
    }

    fn get_employee_type() -> Result<Access, String> {
        println!("employee type possible values:");
        println!("mang/markt/maintn/spv/staff/tech");
        let mut input = String::new();
        input.clear();
        println!("employee type: ");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line.");
        let empl_type: Access = Self::get_access_type(&input)?;
        return Ok(empl_type);
    }

    fn create_employee_type() -> Access {
        loop {
            let employee_type = Self::get_employee_type();
            match employee_type {
                Ok(result) => return result,
                Err(_) => {
                    continue;
                }
            }
        }
    }

    fn employment_status() -> bool {
        let mut input = String::new();
        loop {
            input.clear();
            println!("employed(yes/no): ");
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line.");
            match input.to_lowercase().trim() {
                "yes" => {
                    return true;
                }
                "no" => return false,
                "" => {
                    println!("empty values not allowed. (yes/no)");
                    continue;
                }
                _ => {
                    println!("only (yes/no) allowed.");
                    continue;
                }
            }
        }
    }

    fn has_acces(&self) -> Result<bool, bool> {
        let mut access = false;
        match self.employee_type {
            Access::Marketing(true) => access = true,
            Access::Maintenance(true) => access = true,
            Access::Manager(true) => access = true,
            _ => {}
        }
        match access && self.employed_status {
            true => Ok(true),
            false => Err(false),
        }
    }

    fn print_access(&self) {
        println!("{} has access: {:?}", self.name, self.has_acces());
    }

    fn create_employee() -> Self {
        Self {
            name: Self::get_name(),
            employee_type: Self::create_employee_type(),
            employed_status: Self::employment_status(),
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
        println!("employee no {i}");
        database.push(Employee::create_employee());
    }
    database
}

fn print_database(vector: &Vec<Employee>) {
    for i in vector {
        println!("{:?}", i);
        println!("{:?} has acces: {:?}", i.name, i.has_acces());
    }
}

fn main() {
    let num_employees = get_num();
    let my_empl_database = empl_database(num_employees);
    print_database(&my_empl_database);
}
