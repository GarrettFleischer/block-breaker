use bevy::{color::palettes::tailwind::SKY_950, prelude::*};

mod ball;
mod physics;

use ball::Ball;

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(Color::from(SKY_950)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, ball::movement)
        .run()
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    Ball::spawn(&mut commands, &mut meshes, &mut materials);
}
