#![allow(unused)]

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("{}: {}", s1, len); // s1 still valid
    
    let mut s2 = s1.clone();
    change(&mut s2);
    println!("{s2}");

    let s3 = &mut s2;
    // if some mutable reference exists to a value (&mut) then
    // no other reference may exist to that same value
    // this throws an error

    // let s4 = &mut s2;
    // println!("{}{}", s3, s4);

    // only allowing one mutable reference allows data races to be caught
    // at compiletime
    // e.g., there cannot be two references that allow writing
    // thus, no data race

    // curlybraces can always be used to make scopes with different references

    let mut s5 = String::from("hi");
    
    {
        let mut s6 = &mut s5; // ok
    }

    let mut s7 = &mut s5;

    // immutable borrows can be duplicated until a mutable borrow is used
    let mut s8 = String::from("Hi");
    let s9 = &s8; // ok
    let s10 = &s8; // ok

    // let s11 = &mut s8; // BAD
    // println!("{} {} {}", s9, s10, s11); // BAD
    
    println!("{} {}", s9, s10);

    // since immutable borrow are not used past this point...
    let s11 = &mut s8; // this is valid
    println!("{}", s11); // this is valid
}

// does not take ownership...
fn calculate_length(s: &String) -> usize { // borrows as reference to s
    // this makes an error, as you cannot modify that which you do not own
    // s.push_str(", world!");
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

// fn dangle() -> &String {
    // let s = String::from("dangleme");
    // &s
// }

fn no_dangle() -> String {
    // always just return by value to avoid dangling pointers
    // the value would otherwise be destroyed at end of scope anyway
    let s = String::from("no_dangle");
    s
}
