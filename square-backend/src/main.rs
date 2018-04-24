extern crate specs;
#[macro_use]
extern crate serde_json;

mod components;
mod systems;

use specs::{ World, DispatcherBuilder };
use systems::ipc_handler::IpcHandler;
use components::Position;

fn main() {
    let mut world = World::new();
    world.register::<Position>();

    let mut dispatcher = DispatcherBuilder::new()
        .add(IpcHandler::new(), "ipc_handler", &[])
        .build();

    loop {
        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }
}
