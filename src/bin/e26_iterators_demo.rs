#![allow(dead_code, unused_variables, unused_imports)]
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // let mut plus_one = vec![];
    // for num in numbers {
    //     plus_one.push(num + 1);
    // }

    // we use type annotation because iter() and collect() operate generically
    let plus_one: Vec<_> = numbers.iter().map(|num| num + 1).collect();
    // filter the numbers greater than 3
    let less_than: Vec<_> = numbers.iter().filter(|num| num <= &&3).collect();
    let find_me = numbers.iter().find(|num| num == &&3);
    println!("find_me: {}", find_me.is_some());
    // number of elements within the iterator
    let count = numbers.iter().count();
    println!("number of elements: {count}");
    // last element in an iterator
    let last = numbers.iter().last();
    println!("last: {:?}", last);
    // min, max
    let min = numbers.iter().min();
    let max = numbers.iter().max();
    println!("min: {:?}, max: {:?}", min, max);
    // take items
    let take: Vec<&i32> = numbers.iter().take(3).collect();
    println!("take elem 3: {:?}", take);
}
