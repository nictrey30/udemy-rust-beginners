use std::collections::HashMap;

// using a structure for Contents just in case I want later to specify, suppose I want to make it that the locker can only have certain things I can add them to the structure
#[derive(Debug)]
struct Contents {
    contents: String,
}

fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(
        1,
        Contents {
            contents: "stuff".to_owned(),
        },
    );
    lockers.insert(
        2,
        Contents {
            contents: "shirt".to_owned(),
        },
    );
    lockers.insert(
        3,
        Contents {
            contents: "gym shorts".to_owned(),
        },
    );
    for (locker_no, contents) in lockers.iter() {
        println!("locker {:?} contains {:?}", locker_no, contents);
    }
}
