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
        user_choice => return Err(user_choice.to_owned()),
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

fn print_choice(choice: &Result<MenuChoice, String>) {
    match choice {
        Ok(choice) => println!("Your choice was: {:?}", choice),
        Err(string) => println!("Your choice was incorrect: {:?}", string),
    }
}

fn main() {
    let input = get_input();
    let choice = get_choice(&input);
    print_choice(&choice);
}
