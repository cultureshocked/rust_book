#![allow(unused)]

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // uncomment this and comment the other join call to see how join() makes the main thread wait.

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // threads are not automatically joined at the end of the scope. if they are still running,
    // they will be aborted mid-execution unless a join is explicitly called.
    handle.join().unwrap(); // thread::spawn returns a JoinHandle object which allows calling
    // .join(). .join() returns a thread::Result<T, E> which will contain the return value of the
    // thread.
    
    let v = vec![1, 2, 3];
    // let handle = thread::spawn(|| println!("Here's a vector: {:?}", v)); // does not compile;
    // because threads are not automatically joined at end of scope, the v object may be
    // invalidated/dropped before the thread can finish execution.
    // we need to move ownership of v to the thread by using the move keyword

    let handle = thread::spawn(move || println!("Here's a vector: {:?}", v));
    handle.join().unwrap(); // commenting out this line actually causes the bug described; the
    // thread will not print in time.

    // same goes for calling drop(v) before the thread can finish running.
    
}
