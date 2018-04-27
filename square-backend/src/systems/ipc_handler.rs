use std::thread;
use std::io::{ stdout, stdin, BufRead, Write };
use std::sync::mpsc::{ channel, Receiver, TryRecvError };
use specs::{ System, Entities, Fetch, LazyUpdate };
use serde_json::{ to_string, from_str, Value, Error };
use components::{ Position, Velocity };

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
        Fetch<'a, LazyUpdate>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (entities, updater): Self::SystemData) {
        let stdout = stdout();

        let json = match self.rx.try_recv() {
            Ok(json) => json,
            Err(e) if e == TryRecvError::Empty => return,
            Err(e) if e == TryRecvError::Disconnected => panic!("Cannot receive input: {}", e),
        };

        let map = match json.as_object() {
            Some(map) => map,
            None => {
                let mut lock = stdout.lock();
                let response = json!({
                    "err": "JSON message is not an object",
                }).to_string();
                lock.write(response.as_bytes());
                return;
            }
        };

        let id = match map.get("id") {
            Some(id) => match id.as_str() {
                Some(id) => id,
                None => {
                    let mut lock = stdout.lock();
                    let response = json!({
                        "err": "value of 'id' key is not a string",
                    }).to_string();
                    lock.write(response.as_bytes());
                    return;
                },
            },
            None => {
                let mut lock = stdout.lock();
                let response = json!({
                    "err": "There is no 'id' key",
                }).to_string();
                lock.write(response.as_bytes());
                return;
            },
        };

        let method = match map.get("method") {
            Some(method) => match method.as_str() {
                Some(method) => method,
                None => {
                    let mut lock = stdout.lock();
                    let response = json!({
                        "id": id,
                        "err": "value of 'method' key is not a string",
                    }).to_string();
                    lock.write(response.as_bytes());
                    return;
                },
            },
            None => {
                let mut lock = stdout.lock();
                let response = json!({
                    "id": id,
                    "err": "There is no 'method' key",
                }).to_string();
                lock.write(response.as_bytes());
                return;
            },
        };

        let params = match map.get("params") {
            Some(params) => match params.as_array() {
                Some(params) => match params.iter() {
                    .map(|p| match p.as_str() {
                        Some(p) => p.to_string(),
                        None => Err("Not all params are string types"),
                    }).collect() {
                        Ok(params) => params,
                        Err(e) => {
                            let mut lock = stdout.lock();
                            let response = json!({
                                "id": id,
                                "err": e,
                            }).to_string();
                            lock.write(response.as_bytes());
                            return;
                        }
                    }
                },
                None => {
                    let mut lock = stdout.lock();
                    let response = json!({
                        "id": id,
                        "err": "value of 'params' key is not a string",
                    }).to_string();
                    lock.write(response.as_bytes());
                    return;
                },
            },
            None => {
                let mut lock = stdout.lock();
                let response = json!({
                    "id": id,
                    "err": "There is no 'params' key",
                }).to_string();
                lock.write(response.as_bytes());
                return;
            },
        };

        let response = match method {
            "spawn" => {
                let entity = entities.create();
                updater.insert(entity, Position { x: 0.0, y: 0.0 });
                updater.insert(entity, Velocity { x: 0.0, y: 0.0 });

                json!({
                    "id": id,
                    "entity_id": entity.id(),
                }).to_string()
            },
            "velocity" => {
                if params.size() < 3 {
                    let mut lock = stdout.lock();
                    let response = json!({
                        "id": id,
                        "err": "too less params",
                    }).to_string();
                    lock.write(response.as_bytes());
                    return;
                }

                let entity_id = params[0].unwrap().parse::<i32>().u;
                let x = params[1].unwrap();
                let y = params[2].unwrap();

                let entity = entities.entity(id);
                velocity.get_mut(entity_id) = Velocity { x: x, y: y };

                json!({
                    "id": id,
                }).to_string()   
            },
            _ => json!({
                "id": id,
                "err": "unknown method",
            }).to_string()
        };

        let mut lock = stdout.lock();
        lock.write(response.as_bytes());
        return;
/*
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
                        "spawn" => {
                            let entity = entities.create();
                            updater.insert(entity, Position { x: 0.0, y: 0.0 });
                            updater.insert(entity, Velocity { x: 0.0, y: 0.0 });

                            json!({
                                "id": id,
                                "entity_id": entity.id(),
                            }).to_string()
                        },
                        "velocity" => {
                            if params.size() < 3 {
                                return json!({
                                    "id": id,
                                    "err": "too less params",
                                });
                            }

                            let entity_id = params[0].unwrap().parse::<i32>().u;
                            let x = params[1].unwrap();
                            let y = params[2].unwrap();

                            let entity = entities.entity(id);
                            velocity.get_mut(entity_id) = Velocity { x: x, y: y };

                            json!({
                                "id": id,
                            })
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
*/
    }
}
