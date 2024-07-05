#![allow(unused)]

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)]
enum List<T> {
    Cons(T, RefCell<Rc<List<T>>>),
    Nil,
}

impl<T> List<T> {
    fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc after b created: {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // because of the cycle, this will crash by overflowing the stack.
    // println!("a next item = {:?}", a.tail());
    // the compiler will not catch bugs like this.

    // in most cases, you can reorganize how ownership works to avoid the actual loop.
    // in other cases, using Weak<T> might be sufficient.

    // a weak ptr is a kind of reference that indicates a value may have already been dropped.
    // the weak_count does not need to be 0 for the Rc to be cleaned up.
    // a weak reference must be upgraded to a strong reference (which returns an Option<Rc<T>>)
    // before it can be used.

    // to illustrate, lets use a tree.
    // a node should own its children, and have some information about its parent.

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // making a temporary weak reference, like null
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // downgrading a reference to weak ref 

    // if we want to use that weak reference,
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade().unwrap()); // unwrap needed as
    // upgrade() returns an Option 
    // this time there is no infinite loop of printing references; just a `(Weak)` reference.

}
