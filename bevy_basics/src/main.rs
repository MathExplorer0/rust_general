use bevy::{prelude::*, render::pass::ClearColor, window::WindowDescriptor};

struct Ball {
    velocity: Vec2,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Sinusoidal Motion".to_string(),
            width: 600.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(ball_movement.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::splat(50.0)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(Ball {
            velocity: Vec2::new(1.0, 1.0),
        });
}

fn ball_movement(time: Res<Time>, mut query: Query<(&mut Transform, &mut Ball)>) {
    for (mut transform, mut ball) in query.iter_mut() {
        let dt = time.delta_seconds();
        // Update position using sinusoidal motion
        transform.translation.x += ball.velocity.x * dt;
        transform.translation.y += ball.velocity.y * dt;
        // Update velocity using sin and cos
        let t = time.seconds_since_startup() as f32;
        ball.velocity.x = (2.0 * t).sin();
        ball.velocity.y = (3.0 * t).cos();
    }
}
