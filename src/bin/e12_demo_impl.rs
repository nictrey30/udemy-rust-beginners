use std::io;

#[derive(Debug)]
struct Temperature {
    degrees_c: f64,
}

impl Temperature {
    fn freezing() -> Self {
        Self { degrees_c: 0.00 }
    }
    fn show_temp(&self) {
        println!("{:?} degrees C", self.degrees_c);
    }
}

fn get_temp() -> f64 {
    let mut value = String::new();
    loop {
        println!("temp: ");
        value.clear();
        io::stdin()
            .read_line(&mut value)
            .expect("failed to read line.");
        let value: f64 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("only numbers allowed");
                continue;
            }
        };
        return value;
    }
}

fn main() {
    let hot = Temperature {
        degrees_c: get_temp(),
    };
    hot.show_temp();
    let cold = Temperature::freezing();
    cold.show_temp();
}
