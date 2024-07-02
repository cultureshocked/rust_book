fn main() {
    println!("Hello, world!");

    another_function();
    let x = five();
    println!("{}", x);
    println!("{}", increment(x));
    println!("{}", increment(x));
}

// no forward declaration needed
fn another_function() {
    println!("Another function");
}

// type after, like in ts
fn five() -> i32 {
    5
}

// pass by value does not break ownership principle as value is copied and function owns copy
fn increment(x: i32) -> i32 {
    x + 1
}
