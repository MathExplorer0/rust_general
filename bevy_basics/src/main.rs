use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResolution};

pub struct CorePlug;

const SCREEN_WIDTH: f32 = 600.0;
const SCREEN_HIEGHT: f32 = 600.0;
const BALL_SCALE: f32 = 30.0;

impl Plugin for CorePlug {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hello bevy".to_string(),
                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HIEGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
}

#[derive(Component)]
struct Ball;

const INIT_BALL_TRANSFORM: Transform =
    Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(BALL_SCALE));

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //camera
    commands.spawn(Camera2dBundle::default());

    //ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0)),
            transform: INIT_BALL_TRANSFORM,
            ..default()
        },
        Ball,
    ));
}

fn main() {
    App::new()
        .add_plugins(CorePlug)
        .add_systems(Startup, setup)
        .run();
}
