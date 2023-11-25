// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

use std::io;

// using tuple structs without named fields to create diffrent types
// struct Color(i32, i32);
#[derive(Debug)]
struct Point(i32, i32);

impl Point {
    fn compare_x_axis(&self, other: &Point) -> bool {
        self.0 == other.0
    }
    fn compare_y_axis(&self, other: &Point) -> bool {
        self.1 == other.1
    }
    fn compare_length(&self, other: &Point) -> bool {
        self.0 > other.0
    }
    fn compare_height(&self, other: &Point) -> bool {
        self.1 > other.1
    }
}

fn get_integer_value(str: &str) -> i32 {
    println!("get {}", str);
    let mut value = String::new();
    loop {
        println!("input an integer value between -100 and 100: ");
        value.clear();
        io::stdin()
            .read_line(&mut value)
            .expect("failed to read line.");
        let value: i32 = match value.trim().parse() {
            Ok(num) => {
                if num < -100 {
                    println!("under -100 values not allowed");
                    continue;
                } else if num > 100 {
                    println!("no more than 100");
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("only integers between -100 & 100 allowed");
                continue;
            }
        };
        return value;
    }
}

fn create_point(x: i32, y: i32) -> Point {
    Point(x, y)
}

fn display_comp_result(point1: &Point, point2: &Point) -> String {
    let mut result = String::new();

    if point1.compare_x_axis(&point2) && point1.compare_y_axis(&point2) {
        result.push_str("point1 has the same coord as point2.");
    } else if point1.compare_x_axis(&point2) && !point1.compare_y_axis(&point2) {
        if point1.compare_height(&point2) {
            result.push_str("point1 has the same X-coord as point2 and it's higher than point2.")
        } else {
            result.push_str("point1 has the same X-coord as point2 and it's lower than point2.")
        }
    } else if !point1.compare_x_axis(&point2) && point1.compare_y_axis(&point2) {
        if point1.compare_length(&point2) {
            result.push_str("point1 is further than point2 and has the same Y-coord as point 2.")
        } else {
            result.push_str("point1 is closer than point2 and has the same Y-coord as point 2.")
        }
    } else {
        if point1.compare_height(&point2) && point1.compare_length(&point2) {
            result.push_str("point1 is both higher and further than point2.");
        } else if point1.compare_height(&point2) && !point1.compare_length(&point2) {
            result.push_str("point1 is higher but closer than point2.");
        } else if !point1.compare_height(&point2) && point1.compare_length(&point2) {
            result.push_str("point1 is lower but further than point2.");
        } else {
            result.push_str("point1 is both lower and closer than point2.");
        }
    }
    result
}

fn main() {
    println!("define point1: ");
    let point1 = create_point(get_integer_value("X"), get_integer_value("Y"));
    println!("define point2: ");
    let point2 = create_point(get_integer_value("X"), get_integer_value("Y"));
    let display_result: String = display_comp_result(&point1, &point2);
    println!("height comp:{}", point1.compare_height(&point2));
    println!("lengh comp:{}", point1.compare_length(&point2));
    println!(
        "point1: {:?} vs point2: {:?} comparison \n {display_result}",
        point1, point2
    );
}
