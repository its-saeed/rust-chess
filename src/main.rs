use bevy::prelude::*;

mod startup;

trait Shape {
    fn area(&self) -> u32;
}

struct Rectangle {
    x: u32,
    y: u32,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (self.radius * self.radius * 3.14) as u32
    }
}

fn main() {
    App::build()
    .add_resource(Msaa {samples: 4})
    .add_resource(WindowDescriptor {
        title: "Chess!".to_string(),
        width: 1600.0,
        height: 1600.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(startup::setup.system())
    .add_startup_system(startup::create_board.system())
    .add_startup_system(startup::create_pieces.system())
    .run();
}
