// survey program
struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}
fn main() {
    let response = Survey {
        q1: Some(12),
        q2: Some(true),
        q3: Some("A".to_owned()),
    };
    match response.q1 {
        Some(answ) => println!("q1: {:?}", answ),
        None => println!("q1: no answer"),
    }
    match response.q2 {
        Some(answ) => println!("q2: {:?}", answ),
        None => println!("q2: no answer"),
    }
    match response.q3 {
        Some(answ) => println!("q3: {:?}", answ),
        None => println!("q3: no answer"),
    }
}
