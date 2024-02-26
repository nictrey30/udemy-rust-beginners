use std::io;

struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("pages = {:?}", book.pages);
}

fn display_rating(book: &Book) {
    println!("rating = {:?}", book.rating);
}

fn get_input(input_type: &str, limit: Option<bool>) -> i32 {
    let mut buffer = String::new();
    println!("input no of {input_type}");
    loop {
        while io::stdin().read_line(&mut buffer).is_err() {
            println!("data error");
        }
        match buffer.trim().parse() {
            Ok(num) => {
                if limit.unwrap_or(false) {
                    if num > 5 || num < 1 {
                        println!("rating out of range");
                        buffer.clear();
                        continue;
                    }
                }
                return num;
            }
            Err(_) => {
                println!("only numbers allowed");
                buffer.clear();
                continue;
            }
        }
    }
}

fn create_book() -> Book {
    Book {
        pages: get_input("pages", Some(false)),
        rating: get_input("rating", Some(true)),
    }
}

fn main() {
    let my_book = create_book();
    display_page_count(&my_book);
    display_rating(&my_book);
}
