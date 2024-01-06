//!Basic thread pool implementation using some code from the rust book.
//!
//!Basic usage:
//!```
//!//create a new thread pool with the specified number of threads
//!let workers: ThreadPool = ThreadPool::new(4);
//!
//!//execute a closure from the thread pool
//!workers.execute(|| {
//!    println!("Hello from a worker thread!");
//!});
//!```

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Res<T> = Result<T, Box<dyn std::error::Error>>;

type Job = Box<dyn FnOnce() + Send + 'static>;

#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F) -> Res<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job)?;

        Ok(())
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            if let Ok(reciever_channel) = receiver.lock() {
                if let Ok(job) = reciever_channel.recv() {
                    job();
                }
            }
        });

        Worker { id, thread }
    }
}
