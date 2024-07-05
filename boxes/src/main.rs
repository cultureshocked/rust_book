#![allow(unused)]

use std::ops::Deref;

struct MyBox<T> (T); // tuple-struct

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// to assist with coercion, we let the compiler dereference this object by implementing the Deref
// trait for MyBox
impl<T> Deref for MyBox<T> {
    type Target = T; // defines the target type that should be returned

    fn deref(&self) -> &Self::Target { // returns an actual reference of type Target
        &self.0 // reference to the data which will get dereferenced by the * operator
    }
}

// boxes are useful for recursive types like this list,
// as their size is known at compile-time, while a recursive struct will vary 
// depending on the depth of recursion.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let b = Box::new(5); // type can be inferred by interior type of box
    println!("b = {b}");


    // atrocious syntax, but this forms the conslist (1, (2, (3, Nil)));
    let list = List::Cons(1, Box::new(
        List::Cons(2, Box::new(
            List::Cons(3, Box::new(
                List::Nil)
            ))
        ))
    );

    // while the compiler will do its best to coerce types and dereference them for us, we can also
    // be explicit during our dereferencing/referencing
    let x = 5;
    let y = &x;
    println!("x = {x}");
    println!("y = {}", y); // coerced into dereference
    println!("*y = {}", *y); // actual dereference
    println!("DBG: y = {:?}", y); // coerced into dereference
    println!("DBG: *y = {:?}", *y); // actual dereference
    // all of these print the same value

    let y = Box::new(x); // we place x in the box which does not take ownership of the original
    // data, but clones the original data. x is still valid.

    println!("y on heap: {y}");
    println!("x = {x}");

    let x = 5;
    let y = MyBox(x);
    println!("x = {x}");
    println!("*y = {}", *y); // implicitly calls the .deref() method on MyBox.
    // or rather, *y expands to *(y.deref())

    let name = MyBox(String::from("Guest"));
    hello(&name); // this one is more complicated
    // name has a type MyBox<String> and we want the function to take in a &str or &String
    // the compiler understands that it can use .deref() to get a &String because of how we defined
    // the Target, dereference it, and take
    // a slice to get an &str. all of this can happen at compile-time so coercion has no additional
    // runtime cost.
    // hello(&name) is equivalent to hello(&(*name)[..]) which is quite a mouthful

    // mutable derefs can be implemented with DerefMut trait which also requires the deref() method
    // and Target type to be defined.

    // box deallocates when it moves out of scope
}

fn hello(s: &str) {
    println!("Hello, {s}!");
}
