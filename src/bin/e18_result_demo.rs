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
    println!("choice -> {:?}", choice);
}

fn main() {
    let input = get_input();
    let choice: Result<MenuChoice, _> = get_choice(&input);
    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("error: {}", e),
    }
}
