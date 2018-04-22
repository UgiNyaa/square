extern crate specs;
extern crate serde_json;

mod components;
mod systems;

use specs::{ World, DispatcherBuilder };
use systems::ipc_handler::IpcHandler;

fn main() {
    let stdin = std::io::stdin();
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .add(IpcHandler::new(), "ipc_handler", &[])
        .build();

    loop {
        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }
}
