#![allow(unused)]

// Rc is not part of the prelude like Box is.
use std::rc::Rc;

// old conslist enum from before, but using reference-counter instead of regular Box
// other side note -- we can make enums generic too...
enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(3, Rc::new(Cons(4, Rc::new(Cons(5, Rc::new(Nil))))))); // Rc to a conslist
    let b = Cons(2, Rc::clone(&a)); // 2 3 4 5
    let c = Cons(4, Rc::clone(&a)); // 4 3 4 5
    // Rc::clone or a.clone() do not actually copy any data; they simply increase the counter
    // internal to the Rc object, and return the reference.

    println!("Rc count of 'a' after creating b and c: {}", Rc::strong_count(&a));

    // make a new scope
    {
        let d = Cons(1, Rc::clone(&a));
        println!("Rc count inside of new scope with d: {}", Rc::strong_count(&a));
    }

    println!("Rc count after extra scope expires and d is invalidated: {}", Rc::strong_count(&a));
    // there is also a Rc::weak_count byt that will include the Weak<T> type which will be
    // encountered later.

    // by the end of this scope, 'a' will no longer have any references, and will be cleaned up
    // itself.

    // Rc is great for sharing immutable (read) references across various parts of the program.
}
