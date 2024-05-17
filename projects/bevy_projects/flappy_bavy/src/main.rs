//WOP
use bevy::prelude::*;

mod core;
use core::*;

mod systems;
use systems::*;

fn main() {
    App::new()
        .add_plugins(CorePlugs)
        .add_systems(Startup, setup)
        .run();
}
