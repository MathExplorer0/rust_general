use bevy::{prelude::*, window::WindowResolution};

pub struct CorePlug;

impl Plugin for CorePlug {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hello bevy".to_string(),
                resolution: WindowResolution::new(600.0, 600.0),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
}

fn main() {
    App::new()
        .add_plugins(CorePlug)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    println!("test");
}
