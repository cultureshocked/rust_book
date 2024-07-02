#![allow(unused)]

// comma separated incl last entry
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// use tuple structs for unnames values in ordered fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs behave like () unit types
struct AlwaysEqual;

fn main() {
    // comma delimited in instantiation as well
    // similar to js
    let mut u1 = User {
        active: true,
        username: String::from("someuname123"),
        email: String::from("uname.example.com"),
        sign_in_count: 1,
    };

    // naive way of instantiating from u1
    // let u2 = User {
        // active: u1.active,
        // username: u1.username,
        // email: String::from("another@example.com"),
        // sign_in_count: u1.sign_in_count,
    // };
    
    // simple way of instantiating from u1
    let u3 = User {
        email: String::from("diff@example.com"),
        ..u1 // this moves existing string value of u1.username
        // u1.username is no logner valid
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // shorthand for username+email as symbol is same as field declaration
        email,
        sign_in_count: 1,
    }
}
