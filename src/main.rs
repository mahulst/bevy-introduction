use bevy::prelude::*;

fn hello_world() {
    println!("hello world!");
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Michel".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Joyce".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Oscar".to_string()));
}

pub struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {:?}", name);
        }
    }
}

struct Person;

#[derive(Debug)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(hello_world.system())
            .add_system(greet_people.system());
    }
}
