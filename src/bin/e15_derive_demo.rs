// automatically implement functionality for enum and struct by using a derive macro
// derive functionality for printing out info
// Clone, Copy -> when applied to a piece of data it informs the compiler that it is allowed to automatically make a copy when you are storing it into a struct or a function, what that means is ownership is no longer transferred when you move the Position enum into a struct or a function(a copy is made instead)
#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };
    // without derive -> using match
    // match me.position {
    //     Position::Manager => println!("manager"),
    //     Position::Supervisor => println!("supervisor"),
    //     Position::Worker => println!("worker"),
    // }

    // with Clone, Copy -> "me" is copied into print_employee and it still exists after the print_employee() is called
    print_employee(me); // Employee { position: Worker, work_hours: 40 } -> first copy of "me"
    print_employee(me); // Employee { position: Worker, work_hours: 40 } -> second copy of "me"
}
