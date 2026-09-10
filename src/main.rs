mod ball;
mod brick;
mod level;
mod paddle;
mod physics;
mod wall;

use ball::Ball;
use bevy::{camera::ScalingMode, color::palettes::tailwind::SKY_950, prelude::*};
use level::Level;
use paddle::Paddle;

const CANVAS_SIZE: Vec2 = Vec2::new(1280., 720.);
const BORDER_SIZE: f32 = 10.;

fn main() -> AppExit {
    App::new()
        .insert_resource(ClearColor(Color::from(SKY_950)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, (paddle::controls, ball::movement))
        .run()
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: CANVAS_SIZE.x + BORDER_SIZE,
                min_height: CANVAS_SIZE.y + BORDER_SIZE,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    Level::spawn(&mut commands, &mut meshes, &mut materials, CANVAS_SIZE);

    Paddle::spawn(&mut commands, Vec2::new(0., -CANVAS_SIZE.y * (3. / 8.)));

    Ball::spawn(&mut commands, &mut meshes, &mut materials);
}
