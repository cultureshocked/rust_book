#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // ::insert() takes ownership of its arguments
    // using bound data makes all those bindings invalid
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores
        .get(&team_name) // access key, returns Option<&V> where V is type of value
        .copied() // converts Option<&T> to Option<T> by using copy()
        .unwrap_or(0); // unwraps Option if is type Some(), or returns 0.
    println!("{team_name} team has scored {score} points");

    // iterate over k/v pairs
    // note that hashmaps are unordered
    for (k, v) in &scores {
        println!("{k}: {v}");
    }

    // insert() can overwrite data
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // if we only want to insert and not overwrite...
    scores.entry(String::from("Blue")) // returns enum Entry representing a value that may/maynot
        // exist
        .or_insert(50); // if doesnt exist, we insert this key/value pair {"Blue", 50}.
        // or_insert() also returns a mutable reference to the value inside Entry enum

    // meaning we can modify values based on old values:
    let blue_score = scores.entry(String::from("Blue"))
        .or_insert(0);
    *blue_score += 1; // update using reference

    println!("Blue score: {}", scores.get("Blue").unwrap());

    // or for a more real-world example:
    let text_data = "hello world wonderful world";
    let mut text_map = HashMap::new();
    for word in text_data.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }
    dbg!(&text_map);
}
