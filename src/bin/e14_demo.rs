// receipts

struct LineItem {
    name: String,
    count: i32,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem {
            name: String::from("fruit"),
            count: 3,
        },
    ];

    for i in &receipt {
        // borrowing the name field using &, and that will create a &str from String
        print_name(&i.name);
        println!("count: {:?}", i.count);
    }
}
