// Topic: Result & the question mark operator
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
// * Maintenance crews
// * Marketing department employees
// * Managers
// * Other employees that work at the company are:
// * Line supervisors
// * Kitchen staff
// * Assembly technicians
// * Ensure that terminated employees cannot access the building regardless of their position

// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are still employed
// * Use a function that returns a Result to determine if the employee may enter the building
// * Print whether the employee may access the building
// * Must use a function that utilizes the question mark operator to do this

use std::io;

#[derive(Debug)]
enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

impl Position {
    fn new() -> Self {
        let mut buffer = String::new();
        println!("possible positions: maint/mrkt/mng/spv/staff/tech");
        println!("choose position: ");
        loop {
            while io::stdin().read_line(&mut buffer).is_err() {
                println!("data error");
            }
            match buffer.to_lowercase().trim() {
                "maint" => return Self::Maintenance,
                "mrkt" => return Self::Marketing,
                "mng" => return Self::Manager,
                "spv" => return Self::LineSupervisor,
                "staff" => return Self::KitchenStaff,
                "tech" => return Self::AssemblyTechnician,
                _ => {
                    println!("only valid positions allowed");
                    buffer.clear();
                    continue;
                }
            }
        }
    }
}

#[derive(Debug)]
enum Status {
    Active,
    Terminated,
}

impl Status {
    // eturn an Active or Terminated status
    fn new() -> Self {
        let mut buffer = String::new();
        println!("choose status A(active) or T(terminated):");
        loop {
            while io::stdin().read_line(&mut buffer).is_err() {
                println!("data error");
            }

            match buffer.to_lowercase().trim() {
                "a" => return Self::Active,
                "t" => return Self::Terminated,
                _ => {
                    println!("only A/T inputs allowed");
                    buffer.clear();
                    continue;
                }
            }
        }
    }
}

struct Employee {
    position: Position,
    status: Status,
}

impl Employee {
    fn new() -> Self {
        Self {
            position: Position::new(),
            status: Status::new(),
        }
    }

    fn access(&self) -> Result<(), String> {
        // checking first to see if employee is still active, using an early return if Status::Terminated
        match self.status {
            Status::Terminated => return Err("access denied".to_owned()),
            _ => (),
        }

        // checking to see if the employee has a valid position to access the building
        match self.position {
            Position::Maintenance => Ok(()),
            Position::Marketing => Ok(()),
            Position::Manager => Ok(()),
            _ => Err("invalid position to acces building.".to_owned()),
        }
    }

    fn print_access(&self) -> Result<(), String> {
        Employee::access(&self)?;
        println!("Acces ok");
        Ok(())
    }

    fn print_employee(&self) {
        println!("position: {:?} status: {:?}", self.position, self.status);
    }
}

fn main() {
    let new_employee = Employee::new();
    if new_employee.print_access().is_err() {
        println!("acces denied");
    };
    new_employee.print_employee();
}
