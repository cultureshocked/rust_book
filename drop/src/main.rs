#![allow(unused)]

struct MySmartPtr {
    data: String,
}

impl Drop for MySmartPtr { // trait included in prelude, no need to bring it into scope like deref
    fn drop(&mut self) {
        println!("Dropping MySmartPtr with data {}", self.data);
    }
}

fn main() {
    let c = MySmartPtr {
        data: "first object".to_string(),
    };

    let d = MySmartPtr {
        data: "second object".to_string(),
    };

    println!("Both ptrs created.");

    // explicit destructor calls are not allowed; this line will not compile
    // c.drop()
    // this is because it will still get called at the end of the scope, which would result in a
    // double-free error. the compiler suggests we use std::mem::drop() to drop the value
    drop(c); // this is also part of the prelude. just know it's at std::mem::drop
    println!("c dropped before end of main() scope");
    // and the message will not print at the end of the scope.

    // both smartptrs c and d are dropped implicitly once the end of scope is reached (and those
    // messages are printed out)
}
