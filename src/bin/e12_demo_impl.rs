use std::io;

#[derive(Debug)]
struct Temperature {
    degrees_c: f64,
}

impl Temperature {
    fn freezing() -> Self {
        Self { degrees_c: 0.0 }
    }
    fn boling() -> Self {
        Self { degrees_c: 100.0 }
    }
    fn show_temp(&self) {
        println!("{:?} degrees C", self.degrees_c);
    }
}

fn get_temp() -> f64 {
    let mut value = String::new();
    println!("temp: ");
    loop {
        while io::stdin().read_line(&mut value).is_err() {
            println!("data error");
        }
        match value.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("only numbers allowed");
                value.clear();
                continue;
            }
        };
    }
}

fn main() {
    let hot = Temperature {
        degrees_c: get_temp(),
    };
    hot.show_temp();
    let cold = Temperature::freezing();
    cold.show_temp();
    let boiling = Temperature::boling();
    boiling.show_temp();
}
