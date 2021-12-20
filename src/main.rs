use bevy::prelude::*;
use bevy_mod_picking::*;

mod startup;
mod pieces;
mod board;

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
    .add_plugin(PickingPlugin)
    .add_startup_system(startup::setup.system())
    .add_startup_system(board::create_board.system())
    .add_startup_system(pieces::create_pieces.system())
    .run();
}
