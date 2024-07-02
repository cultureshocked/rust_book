#![allow(unused)]

fn main() {
    // unlike js -- the object itself cannto be modified unless the `mut` keyword is provided
    // not just the reference
    // this is like `const T * = ` in C rather than `T const * = `
    // s can still be shadowed, though, or with `mut` entirely rebound to a new String::new().
    let mut s = String::new();

    // we can bind a regular stringliteral/stringslice/&str to a variable
    let strslc = "initial_contents";
    let strslc = "initial_contents".to_string(); // or use this method to convert it to String

    let strslc = "initial_contents";
    let mut my_string = String::from(strslc); // and we can use non literals for constructor
    
    //string types are utf-8 encoded, and reminder that chars are 4 bytes/32bit

    // if the string is mutable, we can concatenate (3 ways)
    my_string.push_str("_and"); // takes reference always
    my_string += "_other"; // + operator takes reference: &str or &String
    my_string = format!("{}_data", my_string); // similar to sprintf and returns new string
    println!("{my_string}");

    // if we want a string with static lifetime (&'static str) then use concat! macro
    let mut lo = String::from("lo");
    lo.push('l'); // takes in char as opposed to &str 
    println!("{lo}");

    // this causes a compiler error
    // strings are not array-indexable
    // let first = &my_string[0];
    
    // String is a wrapper over the Vec<u8> type, but we know chars are actually 4 bytes long
    // thus, one char may actually be up to 4 elements of a Vec<u8> and thus, using array indexing
    // is not allowed.
    // it shoudl be noted that slicing into a string is technically allowed, but when using
    // extended utf8 chars, the behaviour is not guaranteed to work.
    // instead, use String::chars() to convert to a range of chars, or ::bytes() to return each
    // individual u8.
    
    for c in my_string.chars() {
        print!("'{c}', ");
    }
    print!("\n");

    for n in my_string.bytes() {
        print!("{n} ");
    }
    print!("\n");
}
