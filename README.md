Basic thread pool implementation using some code from the rust book.

Basic usage:
```rust
//create a new thread pool with the specified number of threads
let workers: ThreadPool = ThreadPool::new(4);

//execute a closure from the thread pool
workers.execute(|| {
    println!("Hello from a worker thread!");
}).unwrap();
```

Go to https://docs.rs/std-threadpool/