// * Print the flavor of a drink and it's fluid ounces

use std::io;

#[derive(Debug)]
enum Flavors {
    Apple,
    Orange,
    Cola,
    Cherry,
}

struct Drink {
    flavor: Flavors,
    size: i32,
}

fn get_flavor() -> Flavors {
    let mut buffer = String::new();
    println!("choose a flavor => apple/orange/cola/cherry: ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data input error");
        }
        match buffer.trim() {
            "apple" => return Flavors::Apple,
            "orange" => return Flavors::Orange,
            "cola" => return Flavors::Cola,
            "cherry" => return Flavors::Cherry,
            _ => {
                println!("apple/orange/cola/cherry only");
                buffer.clear();
                continue;
            }
        }
    }
}

fn get_size() -> i32 {
    let mut buffer = String::new();
    let allowed_sizes = [330, 500, 750, 1000, 1500];
    println!("choose a size => 330/500/750/1000/1500: ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data input error");
        }
        match buffer.trim().parse() {
            Ok(num) => {
                if allowed_sizes.contains(&num) {
                    return num;
                }
                println!("only 330/500/750/1000/1500 allowed.");
                buffer.clear();
            }
            Err(_) => {
                println!("please input only nums.");
                buffer.clear();
            }
        }
    }
}

fn create_drink() -> Drink {
    let size = get_size();
    let flavor = get_flavor();
    Drink { size, flavor }
}

fn print_drink(drink: &Drink) {
    let flavor: String = match drink.flavor {
        Flavors::Apple => "apple".to_owned(),
        Flavors::Cherry => "cherry".to_owned(),
        Flavors::Cola => "cola".to_owned(),
        Flavors::Orange => "orange".to_owned(),
    };
    println!("{}ml {} drink", drink.size, flavor);
}

fn main() {
    let my_drink = create_drink();
    print_drink(&my_drink);
}
