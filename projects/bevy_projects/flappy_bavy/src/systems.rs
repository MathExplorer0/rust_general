use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

static PIPE_POS_TEST: Transform = Transform {
    translation: Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
    rotation: Quat::IDENTITY,
    scale: Vec3 {
        x: 100.0,
        y: 300.0,
        z: 0.0,
    },
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::default())),
        material: materials.add(Color::RED),
        transform: PIPE_POS_TEST,
        ..default()
    });
}

pub fn update_test() {}
