use bevy::prelude::*;
#[derive(Component)]
struct Name(String);
#[derive(Component)]

struct Person;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,(add_people,hello_world))
        .add_systems(Update, greet)
        .run();
}
fn add_people(mut command:Commands) {
    command.spawn(Name("com".to_string()));
    command.spawn(Name("cl".to_string()));
    command.spawn((Name("sss".to_string()), Person));
    command.spawn(Name("com".to_string()));
}
fn hello_world() {
    print!("hello")
    }
fn greet(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}", name.0)
    }
}
