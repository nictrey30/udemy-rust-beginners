#![allow(dead_code, unused_variables, unused_imports)]
fn main() {
    let a: Option<i32> = Some(1);
    dbg!(a);
    // is_some(), is_none() returns bool
    let a_is_some = a.is_some();
    dbg!(a_is_some);
    let a_is_none = a.is_none();
    dbg!(a_is_none);
    // map() - only applies if there is Some() data, if there is no data nothing will happen
    let a_mapped = a.map(|num| num + 1);
    dbg!(a_mapped);
    // filter() - takes a closure that accepts a single function argument which is the inner type of the option, and the body of the closure needs to return true or false. if true we keep the optional data, else discard it.
    // filter borrows the data
    let a_filtered = a.filter(|num| num == &1);
    dbg!(a_filtered);
    // or_else if a is Some() nothing will happen, if a is None then we are going to return Some() new data
    let a_or_else = a.or_else(|| Some(5));
    dbg!(a_or_else);
    // unwrap_or_else() - accepts a closure that takes no argumets. returns a non-optional data
    let unwrapped = a.unwrap_or_else(|| 0);
    dbg!(unwrapped);
}
