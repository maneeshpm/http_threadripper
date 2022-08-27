use std::net::TcpListener;
use std::env;

use http_threadripper::handle_stream;

pub mod multithread;

fn main() {
    let mut args = env::args();
    args.next();
    let mut thread_count = 4;

    match args.next() {
        None => println!("Thread Count not found, starting with default {} threads", thread_count),
        Some(a) => match a.parse::<usize>() {
            Ok(b) => thread_count = b,
            Err(_) => println!("Cannot parse thread count, using default {} threads", thread_count)
        }
    };

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = multithread::ThreadPool::new(thread_count);


    for stream in listener.incoming() {
        let s = stream.unwrap();
        pool.exec(|| handle_stream(s));        
    }
}
