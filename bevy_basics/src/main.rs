use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Vladimir".to_string())));
    commands.spawn((Person, Name("Olivia".to_string())));
    commands.spawn((Person, Name("Jack".to_string())));
}

fn hello_world() {
    println!("Hello W");
}

fn greet_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Jack" {
            name.0 = "ch3ar".to_string();
            println!("{}", name.0);
            break;
        }
        println!("{}", name.0);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}
