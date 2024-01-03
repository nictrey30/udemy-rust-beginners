#![allow(dead_code, unused_variables, unused_imports)]
// Topic: Traits
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable represents the length of a side.
// * Print out the perimeter of the shapes

// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn calc_perimeter(&self) -> i32;
}

struct Square {
    length: i32,
}
impl Perimeter for Square {
    fn calc_perimeter(&self) -> i32 {
        self.length * 4
    }
}

struct Triangle {
    laterals: [i32; 3],
}
impl Perimeter for Triangle {
    fn calc_perimeter(&self) -> i32 {
        let mut perim: i32 = 0;
        for lateral in self.laterals {
            perim += lateral;
        }
        perim
    }
}

fn perimeter(object: impl Perimeter) -> i32 {
    object.calc_perimeter()
}

fn main() {}

#[cfg(test)]

mod test {
    use crate::{perimeter, Square, Triangle};

    #[test]
    fn calc_perimeter_square() {
        let square = Square { length: 5 };
        let perimeter = perimeter(square);
        assert_eq!(
            perimeter, 20,
            "perimeter of a square with length 4 should be 20"
        );
    }

    #[test]
    fn calc_triangle_square() {
        let traingle = Triangle {
            laterals: [4, 8, 6],
        };
        let perimeter = perimeter(traingle);
        assert_eq!(
            perimeter, 18,
            "perimeter of a traingle with laterals of: 4, 8, 6 should be 18"
        );
    }
}
