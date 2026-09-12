use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Future<T> {
    rx: Arc<Mutex<Receiver<T>>>,
}

impl<T> Future<T> {
    pub fn try_get(&self) -> Option<T> {
        self.rx.lock().unwrap().try_recv().ok()
    }
}

pub struct Worker {
    tx: Option<Sender<Job>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Job>();

        let handle = thread::spawn(move || {
            while let Ok(job) = rx.recv() {
                job();
            }
        });

        Worker {
            tx: Some(tx),
            handle: Some(handle),
        }
    }

    pub fn execute<F, T>(&self, f: F) -> Future<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (result_tx, result_rx) = mpsc::sync_channel(1);
        let job = Box::new(move || {
            let result = f();
            let _ = result_tx.send(result);
        });

        self.tx.as_ref().unwrap().send(job).unwrap();

        Future {
            rx: Arc::new(Mutex::new(result_rx)),
        }
    }
}
impl Drop for Worker {
    fn drop(&mut self) {
        self.tx.take();
        if let Some(h) = self.handle.take() {
            h.join().ok();
        }
    }
}
