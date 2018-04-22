use specs::{ System };

pub struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {
        println!("Hello World!");
    }
}
