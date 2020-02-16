use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Message {
    i: i32,
}

pub struct Backend {
    name: String,
    tx: Option<Sender<Message>>,
}

impl Backend {
    pub fn new(name: &str) -> Backend {
        Backend { name: String::from(name), tx: None }
    }
}

impl Drop for Backend {
    fn drop(&mut self) {
        println!("Dropping backend. TODO: actually clean up threads and channels!");
    }
}

impl Backend {
    pub fn start(&mut self) {
        let (tx, rx) = mpsc::channel();
        let name = self.name.clone();
        thread::spawn(move || {
            let mut counter = 0;
            for m in rx.iter() {
                counter += 1;
                println!("[{}] Got {:?} (message no. {})", name, m, counter);
            }
        });
        self.tx = Some(tx);
    }
    pub fn spawn_poker(&self, delay_ms: i32, msg: i32) {
        if let Some(tx) = self.tx.clone() {
            thread::spawn(move || {
                let delay = Duration::from_millis(delay_ms as u64);
                thread::sleep(delay);
                tx.send(Message { i: msg }).expect("oops, cannot send!");
            });
        } else {
            println!("hold your horses, no channel to transmit available!");
        }
    }
    pub fn poke(&self, msg: i32) {
        if let Some(tx) = self.tx.clone() {
           tx.send(Message { i: msg }).expect("oops, cannot send!");
        } else {
            println!("hold your horses, no channel to transmit available!");
        }
    }
}
