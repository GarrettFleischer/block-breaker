use bevy::{color::palettes::tailwind::SLATE_950, prelude::*};

use super::physics::Velocity;

pub const BALL_SIZE: f32 = 10.;

#[derive(Debug, Component)]
pub struct Ball;

impl Ball {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((
            Ball,
            Velocity::new(-200., -400.),
            Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
            MeshMaterial2d(materials.add(Color::from(SLATE_950))),
            Transform::from_xyz(0., 0., 0.),
            children![(
                Mesh2d(meshes.add(Circle::new(BALL_SIZE - 1.))),
                MeshMaterial2d(materials.add(Color::WHITE)),
                Transform::from_xyz(0., 0., 1.)
            )],
        ));
    }
}

pub fn movement(mut balls: Query<(&mut Transform, &Velocity), With<Ball>>, time: Res<Time>) {
    for (mut transform, velocity) in &mut balls {
        let movement_this_frame = velocity.0 * time.delta_secs();
        transform.translation += movement_this_frame.extend(0.);
    }
}
