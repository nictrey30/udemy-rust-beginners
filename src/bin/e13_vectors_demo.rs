use std::io;

#[derive(Debug)]
struct Test {
    score: i32,
}

impl Test {
    fn new() -> Self {
        let mut buffer = String::new();
        println!("score (between 1 and 100): ");
        loop {
            while io::stdin().read_line(&mut buffer).is_err() {
                println!("data error");
            }
            match buffer.trim().parse() {
                Ok(num) => {
                    if num < 1 || num > 100 {
                        println!("num out of range 1..100");
                        buffer.clear();
                        continue;
                    }
                    return Self { score: num };
                }
                Err(_) => {
                    println!("must be a no (between 1 and 100)");
                    buffer.clear();
                    continue;
                }
            }
        }
    }
}

fn main() {
    let mut my_scores = vec![];
    for i in 0..5 {
        println!("test score {i}");
        my_scores.push(Test::new());
    }
}
