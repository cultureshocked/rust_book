#![allow(unused)]

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let m = Mutex::new(5);
    println!("mutex value before modification: {m:?}");
    {
        let mut num = m.lock().unwrap(); // lock returns a reference (MutexGuard)
                                         // .lock() is a blocking operation, naturally
                                         // if another thread panics while owning the lock, .lock() will fail automatically, and
                                         // unwrap() will trigger a panic of its own.
        *num = 6;
    }
    println!("mutex value after modification: {m:?}");

    let counter = Arc::new(Mutex::new(0)); // Arc is Atomic Rc, which incurs runtime overhead but
    // is thread-safe.
    let mut handles = vec![];
    for _ in 0..10 {
        let thread_counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = thread_counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result of counter: {counter:?}");

    // there are two important traits that objects must have to be used in threaded contexts
    // Send means that passing of ownership of an object is safe between threads.
    // Sync means that passing immutable references between threads is safe.
}
