#![allow(unused)]

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // do not communicate by sharing memory; share memory by communicating
    // rust implements sending messages between threads by using channels.
    // a channel has a receiver and a transmitter (rx/tx)
    // a channel is open if both the transmitter and receiver are valid
    // if either transmitter or receiver get dropped, the channel is closed/invalid

    // mpsc = multiple producer, single consumer
    let (tx, rx) = mpsc::channel(); // may require a type, but can be inferred by what values the
    // tx/rx pair uses later in the code. channels are a type of generic.

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // .send() takes ownership of the message being sent.
    });

    let received = rx.recv().unwrap(); // .recv() blocks until something is received and it will be
    // type Result<T, E>. the err will be returned when the channel closes (e.g., tx is dropped.)

    // another useful method is .try_recv() which will not block, and will return some Err() if there is
    // nothing in the receiver buffer.

    println!("Received msg '{}' from thread", received);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            "hi".to_string(),
            "from".to_string(),
            "the".to_string(),
            "thread".to_string(),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // we can use the rx object as an iterator as well, which reaches its end once the channel
    // closes.
    for received in rx {
        println!("Received message: '{received}'");
    }

    // as mpsc means multiple producers, this means we can clone the transmitter.
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            "hi".to_string(),
            "from".to_string(),
            "the".to_string(),
            "thread".to_string(),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            "more".to_string(),
            "messages".to_string(),
            "for".to_string(),
            "you".to_string(),
        ];

        for val in vals {
            tx1.send(val).unwrap(); // using the cloned transmitter
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Received message: '{received}'");
    }
}
