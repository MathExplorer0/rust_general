use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, hello_world)
        .run();
}

fn hello_world() {
    println!("Hello, World");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}
