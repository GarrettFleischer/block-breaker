use bevy::{
    color::palettes::tailwind::{SKY_50, SKY_800},
    prelude::*,
};

use super::wall::Wall;

pub const BORDER_SIZE: f32 = 4.;

#[derive(Debug, Component)]
pub struct Level;

impl Level {
    pub fn spawn(commands: &mut Commands, size: Vec2) {
        commands.spawn((
            Sprite {
                custom_size: Some(Vec2::new(
                    // 4px border
                    size.x + BORDER_SIZE,
                    size.y + BORDER_SIZE,
                )),
                color: Color::from(SKY_50),
                ..default()
            },
            Transform::from_xyz(0., 0., -3.0),
        ));
        commands.spawn((
            Sprite {
                custom_size: Some(size),
                color: Color::from(SKY_800),
                ..default()
            },
            Transform::from_xyz(0., 0., -2.0),
        ));

        Wall::spawn(commands, Vec2::X, Vec2::new(-size.x / 2., 0.));
        Wall::spawn(commands, Vec2::NEG_X, Vec2::new(size.x / 2., 0.));
        Wall::spawn(commands, Vec2::Y, Vec2::new(-size.y / 2., 0.));
        Wall::spawn(commands, Vec2::NEG_Y, Vec2::new(size.y / 2., 0.));
    }
}
