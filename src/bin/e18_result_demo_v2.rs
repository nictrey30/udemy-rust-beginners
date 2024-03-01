// working with Result - demo

use std::io;

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input.to_lowercase().trim() {
        "main" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("choice not found".to_owned()),
    }
}

fn pick_choice() -> Result<(), String> {
    // the ? operator will automatically perform a match on Result
    // if it is an Ok the data will be placed in the variable choice, if it is an Err it will be returned automatically
    let choice: MenuChoice = get_choice(&get_input())?;
    print_choice(&choice);
    Ok(())
}

fn get_input() -> String {
    let mut buffer = String::new();
    println!("please input a choice: ");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data input error, try again!");
        }
        // checking if the input has 0 length
        if buffer.trim().chars().count() == 0 {
            println!("choice cannot be empty");
            buffer.clear();
            continue;
        }
        return buffer.trim().to_lowercase().to_owned();
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("{:?}", choice);
}

fn main() {
    // let choice: Result<MenuChoice, String> = get_choice(&get_input());
    // match choice {
    //     Ok(inner_choice) => print_choice(&inner_choice),
    //     Err(e) => println!("{:?}", e),
    // }
    let choice = pick_choice();
    println!("choice = {:?}", choice);
}
