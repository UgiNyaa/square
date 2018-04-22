use std::thread;
use std::io::{ stdin, BufRead };
use std::sync::mpsc::{ channel, Sender, Receiver, TryRecvError };
use specs::System;
use serde_json::{ from_str, Value, Error };

pub struct InputReceiver {
    rx: Receiver<Value>,
}

impl InputReceiver {
    pub fn new() -> Self {
        let (sx, rx) = channel::<Value>();

        thread::spawn(move || {
            let stdin = stdin();
            for line in stdin.lock().lines() {
                sx.send(from_str(&line.unwrap()).unwrap());
            }
        });

        InputReceiver {
            rx: rx,
        }
    }
}

impl<'a> System<'a> for InputReceiver {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {
        match self.rx.try_recv() {
            Ok(v) => {
                println!("input received: {:?}", v);
            }
            Err(e) => {
                match e {
                    TryRecvError::Empty => { }
                    TryRecvError::Disconnected => panic!("Cannot receive input: {}", e)
                }
            }
        }
    }
}
