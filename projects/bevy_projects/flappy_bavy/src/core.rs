use bevy::{prelude::*, window::WindowResolution};

pub const WIN_WIDTH: f32 = 500.0;
pub const WIN_HEIGHT: f32 = 600.0;

pub struct CorePlugs;

impl Plugin for CorePlugs {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy bevy".to_string(),
                        resolution: WindowResolution::new(WIN_WIDTH, WIN_HEIGHT),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        );
    }
}
