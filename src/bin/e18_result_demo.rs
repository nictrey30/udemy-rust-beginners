// working with Result - demo

use std::io;

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(choice: &str) -> Result<MenuChoice, String> {
    match choice {
        "mainmenu" => return Ok(MenuChoice::MainMenu),
        "start" => return Ok(MenuChoice::Start),
        "quit" => return Ok(MenuChoice::Quit),
        _ => return Err("menu choice not found".to_owned()),
    }
}

fn get_input() -> String {
    let mut input: String = String::new();
    loop {
        input.clear();
        println!("enter your choice: mainmenu/start/quit:");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line.");
        match input.to_lowercase().trim() {
            "" => {
                println!("empty inputs not allowed.");
                continue;
            }
            user_input => return user_input.to_owned(),
        }
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice: {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    // by using ?, get_choice instead of returning a Result, will automatically perform a match operation
    // if the Result is an Ok variant, then the inner data of Ok will be returned
    // if it is the Err variant the function will returl a Result Err variant aka String in our case, otherwise we get the inner data
    let choice: MenuChoice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}

fn main() {
    let input = get_input();
    // the choice will always gonna be a Result, because pick_choice always returns a Result
    let choice = pick_choice(&input);
    println!("choice value = {:?}", choice);
}
