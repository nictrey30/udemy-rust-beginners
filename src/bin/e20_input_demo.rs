use std::io;

// the Result err type it is already provided for io::Result, so we only need to provide the successful type
fn get_input() -> io::Result<String> {
    println!("input: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn main() {
    let mut all_input = vec![];
    let mut times_input = 0;
    while times_input < 2 {
        match get_input() {
            Ok(words) => {
                all_input.push(words);
                times_input += 1;
            }
            Err(e) => println!("error: {:?}", e),
        }
    }
    for input in &all_input {
        println!(
            "Original: {:?}, capitalized: {:?}",
            input,
            input.to_uppercase()
        );
    }
}
