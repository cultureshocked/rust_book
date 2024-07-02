fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    //
    let y = s; // y now "owns" reference of s and s is no longer valid
    // println!("{}", s) // borrow error
    // effectively same as using std::move in cpp; not a copy or shallow copy

    // if we want to continue using old references, we need to actually perform a deep copy using
    // .clone()
    let s = y.clone();
    println!("{}", s);
    
    // this pattern only applies to heap-allocated data
    // stack data works without needing to clone
    let x = 5;
    let z = x;
    println!("x: {}, z: {}", x, z);
    
    // types may implement a `Copy` trait which indicates they are trivially copyable
    // this includes ints, bools, floats, chars, and tuples which only use the previous types
    takes_ownership(s);
    makes_copy(z);

    // because of ownership, the reference to s is no longer valid
    // println!("{}", s);
    
    let s = gives_ownership();
    println!("{s}");
    let s = takes_and_gives_back(s);
    println!("{s}");

    // rememebr to use () for destructuring
    let (s, len) = calc_length(s);
    println!("{}: {}", s, len);
}

// this function takes ownership of the argument because...
fn takes_ownership(some_string: String) { // strings are not trivially copyable
    println!("{some_string}");
}

// this function does not take ownership because...
fn makes_copy(some_int: i32) { // ints are trivially copyable
    println!("{some_int}");
}

fn gives_ownership() -> String {
    let s = String::from("helloworld");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calc_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
