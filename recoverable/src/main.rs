#![allow(unused)]

use std::fs::File;
use std::io::{ErrorKind, Read, self};

fn main() {
    let greeting_file_result = File::open("hello.txt");
    dbg!(&greeting_file_result);

    // this might seem like hell because match statements are fairly primitive and require handling
    // all cases possible.
    // this can be fixed later on by using closures along with other Result:: methods.
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error}");
            }
        },
    };

    let greeting_file_result = File::open("hello.txt").unwrap(); // .unwrap() will automatically
    // panic!() if Result is of type Err().
    
    // expect() can pass a message to the panic! to help tracking down problems.
    let greeting_file_result = File::open("hello2.txt")
        .expect("hello2.txt should be included in this project");
    // otherwise just returns the file handle

    let username_result = read_username_from_file().expect("username load encountered an error.");
    println!("{username_result}");

}

// the ? operator will validate whether a Result is OK().
// if the Result is Err(), then it will returns the Err() from the entire function without
// proceeding.
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?; // problem with open?
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // problem with read?
    Ok(username)
}
