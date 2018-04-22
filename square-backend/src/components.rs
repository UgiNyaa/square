use specs::{ Component, VecStorage };

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
