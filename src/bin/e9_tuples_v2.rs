// Topic: Data management using tuples
// Requirements:
// * Print whether the y-value of a cartesian coordinate is greater than 5, less than 5, or equal to 5
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

use std::cmp::Ordering;
use std::io;

// using tuple structs without named fields to create diffrent types
#[derive(Debug)]
struct Point(i32, i32);

fn get_coord(coord_type: &str) -> i32 {
    let mut buffer = String::new();
    println!("coord {coord_type}:");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data error");
        }
        match buffer.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("please enter only numbers");
                buffer.clear();
                continue;
            }
        }
    }
}

fn create_point() -> Point {
    Point(get_coord("x"), get_coord("y"))
}

fn display_point(point: &Point) {
    println!("point => x: {}, y: {}", point.0, point.1);
}

fn compare_coord_y(point: &Point) {
    let Point(x, y) = point;
    let num = get_coord("to compare");
    match x.cmp(&num) {
        Ordering::Equal => println!("x coord is equal to {num}"),
        Ordering::Less => println!("x coord is less than {num}"),
        Ordering::Greater => println!("x coord is greater than {num}"),
    }
    match y.cmp(&num) {
        Ordering::Equal => println!("y coord is equal to {num}"),
        Ordering::Less => println!("y coord is less than {num}"),
        Ordering::Greater => println!("y coord is greater than {num}"),
    }
}

fn main() {
    let my_point = create_point();
    display_point(&my_point);
    compare_coord_y(&my_point);
}
