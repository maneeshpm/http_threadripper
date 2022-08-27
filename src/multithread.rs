use std::thread;
use std::sync::{mpsc, Arc, Mutex};

type Job = Box<dyn Send + 'static + FnOnce()>;

pub struct Worker {
    _id: usize,
    _handler: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handler = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker id {} received job", id);
            job();
            println!("Worker id {} finished job", id);
        });
        Worker{_id: id, _handler: handler}
    }
}

pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    pub fn new(count: usize) -> ThreadPool {
        assert!(count > 0);
        let mut workers = Vec::with_capacity(count);

        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..count {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool{_workers: workers, sender}
    }

    pub fn exec<F>(&self, f: F)
    where
        F: Send + FnOnce() + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
