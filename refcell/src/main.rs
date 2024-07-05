#![allow(unused)]

#[derive(Debug)]
enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}

use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

fn main() {
    let val = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&val), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)); // 3 5 nil
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)); // 4 5 nil

    *val.borrow_mut() += 10; // lists are not [3 15], [4 15]
    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    // just note, refcell is not thread-safe.
    // the thread-safe equivalent of refcell is Mutex<T>.
}
