use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResolution};

pub struct MainPlug;

impl Plugin for MainPlug {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Learning Bevy".to_string(),
                resolution: WindowResolution::new(600.0, 600.0),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.0)),
        material: materials.add(Color::PURPLE),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(MainPlug)
        .add_systems(Startup, setup)
        .run();
}
