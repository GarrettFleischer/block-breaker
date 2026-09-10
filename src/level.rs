use super::brick::{BRICK_SIZE, Brick};
use super::wall::Wall;
use bevy::{
    color::palettes::tailwind::{SKY_50, SKY_400, SKY_800},
    prelude::*,
};

pub const BORDER_SIZE: f32 = 4.;

#[derive(Debug, Component)]
pub struct Level {
    pub size: Vec2,
}

impl Level {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        size: Vec2,
    ) {
        commands.spawn(Level { size: size });

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
        Wall::spawn(commands, Vec2::Y, Vec2::new(0., -size.y / 2.));
        Wall::spawn(commands, Vec2::NEG_Y, Vec2::new(0., size.y / 2.));

        let num_bricks_per_row = 13;
        let rows = 6;
        let base_color = Oklcha::from(SKY_400);
        for row in 0..rows {
            for col in 0..num_bricks_per_row {
                let color: Color = base_color
                    .with_hue(((row + col) % 8) as f32 * (num_bricks_per_row * rows) as f32)
                    .into();

                let x = BRICK_SIZE.x * col as f32 - BRICK_SIZE.x * num_bricks_per_row as f32 / 2.
                    + BRICK_SIZE.x / 2.;
                let y = size.y * (3. / 8.) - BRICK_SIZE.y * row as f32;

                Brick::spawn(commands, meshes, materials, Vec2::new(x, y), color);
            }
        }
    }
}
