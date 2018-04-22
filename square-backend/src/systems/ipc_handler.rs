use std::thread;
use std::io::{ stdin, BufRead };
use std::sync::mpsc::{ channel, Sender, Receiver, TryRecvError };
use specs::System;
use serde_json::{ from_str, Value, Error };

pub struct IpcHandler {
    rx: Receiver<Value>,
}

impl IpcHandler {
    pub fn new() -> Self {
        let (sx, rx) = channel::<Value>();

        thread::spawn(move || {
            let stdin = stdin();
            for line in stdin.lock().lines() {
                sx.send(from_str(&line.unwrap()).unwrap());
            }
        });

        IpcHandler {
            rx: rx,
        }
    }
}

impl<'a> System<'a> for IpcHandler {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {
        let result = self.rx.try_recv();

        if let Err(e) = result {
            match e {
                TryRecvError::Empty => return,
                TryRecvError::Disconnected => panic!("Cannot receive input: {}", e),
            }
        }

        if let Ok(json) = result {
            println!("input received: {:?}", json);
        }
    }
}
