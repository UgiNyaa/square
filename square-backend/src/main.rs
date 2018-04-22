extern crate specs;
extern crate serde_json;

mod components;
mod systems;

use specs::{ World, DispatcherBuilder };
use systems::input_receiver::InputReceiver;

fn main() {
    let stdin = std::io::stdin();
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .add(InputReceiver::new(), "input_receiver", &[])
        .build();

    loop {
        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }
}
