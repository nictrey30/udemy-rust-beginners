use std::io;

// the Result err type it is already provided for io::Result, so we only need to provide the successful type
fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    // the function read_line is going to read a line and save that line into that buffer that is bororwing
    // "?" operator: if the function fails we will return an error from the function, otherwise the data will be available in our buffer
    io::stdin().read_line(&mut buffer)?;

    // return the data... trim() creates a slice and we need to take back ownership
    Ok(buffer.trim().to_lowercase().to_owned())
}

fn check_input(my_input: &str) -> bool {
    // println!("{}", my_input);
    if my_input == "n" {
        return false;
    }
    true
}

fn main() {
    let mut all_input = vec![];
    let mut answer = true;
    while answer {
        println!("input: (n ends program)");
        match get_input() {
            Ok(input) => {
                if input == "" {
                    println!("empty value. try again");
                    continue;
                }
                if check_input(&input[..]) == false {
                    answer = false;
                }
                if answer == true {
                    all_input.push(input)
                }
            }
            Err(_) => {
                println!("input not valid");
                continue;
            }
        }
    }
    println!("here are the inputs: ");
    for input in all_input {
        println!("{}", input);
    }
}
