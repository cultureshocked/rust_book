#![allow(unused)]

use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug)] 
struct Rectangle {
    width: u32,
    height: u32,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) // uses a closure
        // closure syntax is largely optional because the compiler can infer what type it should
        // operate on based on how it is used.
        //
        // closures are not generic, that is, using a closure on two distinct concrete types in
        // different places will result in a compiler error. the closure's types must be concrete,
        // although they can be inferred.
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let user_pref2 = None;

    let giveaway1 = store.giveaway(user_pref1);
    let giveaway2 = store.giveaway(user_pref2);

    println!("User1 won {:?} shirt", giveaway1);
    println!("User2 won {:?} shirt", giveaway2);

    // closures can capture outside data using:
    // immutable borrowing
    // mutable borrowing
    // taking ownership
    // the compiler will often figure out which one we need, again.
    let list = vec![1, 2, 3, 4];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}"); // println! uses an immutable borrow
    // internally, so this closure will borrow using an immutable reference to list.
    
    // as we are allowed multiple immutable borrows at any point, the following statements work
    // perfectly fine.
    
    // keep in mind -- the borrow happens at definition-time, NOT call-time. the reference already
    // exists inside the closure before any of these statements execute.

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut list = vec![1, 2, 3, 4];
    println!("Before defining the mutable closure: {list:?}");

    let mut borrows_mutably = || list.push(7); // now captures a mutable reference (note we use
    // `mut` keyword on the closure itself)

    // we cannot put any println! here because we already have a mutable reference and that would
    // make the borrow checker scream
    borrows_mutably();
    println!("After calling the closure: {list:?}");

    let list = vec![1, 2, 3, 4];
    println!("Before defining ownership closure: {list:?}");

    thread::spawn(move || println!("From thread closure: {list:?}"))
        .join()
        .unwrap();

    // now this wont compile
    // println!("After ownership closure called: {list:?}");
    // as the object no longer exists once the closure is done executing.

    // for all intents and purposes, closures are defined as a `Fn`, `FnOnce`, or `FnMut` type,
    // which are just function types with some specific conditions about
    // borrowing/mutating/returning data. regular named functions can be used inplace where any of
    // those closure types are used, as well. this enables us to write more concrete higher-order
    // functions.

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width );
    println!("sorted by width: {list:?}");
    list.sort_by_key(|r| r.height );
    println!("sorted by height: {list:?}");
    list.sort_by_key(|r| r.width * r.height );
    println!("sorted by area: {list:?}");

}
