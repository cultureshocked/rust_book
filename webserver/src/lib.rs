#![allow(unused)]

use std::thread;
use std::sync::{Arc, Mutex, mpsc};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    t: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }
        ThreadPool { workers, sender: Some(tx) }
    }

    pub fn execute<F>(&self, f: F) 
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        Worker {
            id,
            t: Some(thread::spawn(move || loop {
                let msg = rx.lock().unwrap().recv();

                match msg {
                    Ok(job) => {
                        println!("Received a job on thread {id}");
                        job();
                    },
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down...");
                        break;
                    }
                }
            })),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(t) = worker.t.take() {
                t.join().unwrap();
            }
        }
    }
}
