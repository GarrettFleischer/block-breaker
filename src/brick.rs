use super::physics::HalfSize;
use bevy::prelude::*;

pub const BRICK_SIZE: Vec2 = Vec2::new(80.0, 40.0);

#[derive(Debug, Component)]
pub struct Brick;

impl Brick {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        position: Vec2,
        color: Color,
    ) {
        commands.spawn((
            Brick,
            Transform::from_xyz(position.x, position.y, 0.),
            Sprite {
                custom_size: Some(BRICK_SIZE),
                color: color.with_alpha(0.4),
                ..default()
            },
            HalfSize(BRICK_SIZE / 2.),
            children![(
                Mesh2d(meshes.add(Rectangle::new(BRICK_SIZE.x - 2., BRICK_SIZE.y - 2.,))),
                MeshMaterial2d(materials.add(Color::from(color)),),
                Transform::from_xyz(0., 0., 1.)
            )],
        ));
    }
}
