use std::io::Write;
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;

fn handle_client(mut stream: TcpStream) {
    stream.write_all(b"Hello world").unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1989").unwrap();
    let workers = ThreadPool::new(4);

    println!("Listening on 127.0.0.1:1989");

    for stream in listener.incoming() {
        workers
            .execute(|| {
                handle_client(stream.unwrap());
            })
            .unwrap();
    }
}
