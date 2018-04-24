use std::thread;
use std::io::{ stdout, stdin, BufRead, Write };
use std::sync::mpsc::{ channel, Sender, Receiver, TryRecvError };
use specs::{ System, Entities, Fetch, LazyUpdate };
use serde_json::{ to_string, from_str, Value, Error };
use components::Position;

pub struct IpcHandler {
    rx: Receiver<Value>,
}

impl IpcHandler {
    pub fn new() -> Self {
        let (sx, rx) = channel::<Value>();

        thread::spawn(move || {
            let stdin = stdin();
            for line in stdin.lock().lines() {
                sx.send(from_str(&line.unwrap()).unwrap()).unwrap();
            }
        });

        IpcHandler {
            rx: rx,
        }
    }
}

impl<'a> System<'a> for IpcHandler {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, LazyUpdate>
    );

    fn run(&mut self, (entities, updater): Self::SystemData) {
        match self.rx.try_recv() {
            Ok(json) => match json.as_object() {
                Some(map) => { 
                    let method = match map.get("method") {
                        Some(method) => match method.as_str() {
                            Some(method) => method,
                            None => panic!("value of 'method' key is not a string"),
                        },
                        None => panic!("There is not 'method' key"),
                    };

                    let id = match map.get("id") {
                        Some(id) => match id.as_str() {
                            Some(id) => id,
                            None => panic!("value of 'id' key is not a string"),
                        },
                        None => panic!("There is not 'id' key"),
                    };

                    let params = match map.get("params") {
                        Some(params) => match params.as_array() {
                            Some(params) => params.iter()
                                .map(|p| match p.as_str() {
                                    Some(p) => p.to_string(),
                                    None => panic!("Not all params are string types"),
                                })
                                .collect::<Vec<String>>(),
                            None => panic!("value of 'params' key is not an array"),
                        },
                        None => panic!("There is not 'params' key"),
                    };

                    println!("method: {:?}, id: {:?}, params: {:?}", method, id, params);

                    let stdout = stdout(); 
                    let response = match method {
                        "join" => {
                            let entity = entities.create();
                            updater.insert(entity, Position { x: 0.0, y: 0.0 });

                            json!({
                                "id": id,
                                "entity_id": entity.id(),
                            }).to_string()
                        },
                        _ => json!({
                            "id": id,
                            "err": "unknown method",
                        }).to_string()
                    };

                    {
                        let mut lock = stdout.lock();

                        lock.write(response.as_bytes()).unwrap();
                    }
                },
                None => panic!("JSON message is not an object"),
            },
            Err(e) => match e {
                TryRecvError::Empty => return,
                TryRecvError::Disconnected => panic!("Cannot receive input: {}", e),
            },
        }
    }
}
