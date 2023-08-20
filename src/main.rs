use bevy::prelude::*;

fn main() 
{
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, greet_people)
        .run();
}

fn add_people(mut commands: Commands)
{
    commands.spawn((Person, Name("Elaina".to_string())));
    commands.spawn((Person, Name("Dril".to_string())));
    commands.spawn((Person, Name("Torin".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>)
{
    for name in &query
    {
        println!("Hello {}!", name.0);
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);