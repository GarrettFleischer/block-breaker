use bevy::{color::palettes::tailwind::SLATE_950, prelude::*};

use super::physics::Velocity;
use super::wall::Wall;

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

pub fn movement(
    mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    walls: Query<(&Wall, &Transform), Without<Ball>>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity) in &mut balls {
        // a ray that casts infinitely in the direction
        // the ball is moving
        let ball_ray = Ray2d::new(
            // the location of the ball
            transform.translation.xy(),
            // the Direction the ball is moving in
            Dir2::new(velocity.0).unwrap(),
        );

        // how far the ball is going to go this frame
        // represented as a vec2
        let ball_movement_this_frame = velocity.0 * time.delta_secs();
        let ball_move_distance = ball_movement_this_frame.length();

        // for each wall, check if we're going to hit it this frame
        for (wall, origin) in walls {
            if let Some(hit_distance) = ball_ray.intersect_plane(origin.translation.xy(), wall.0)
                && hit_distance <= ball_move_distance
            {
                // velocity is just the reflection of the hit
                // this is basically inverting the X or Y direction
                // to move in the opposite direction
                velocity.0 = velocity.0.reflect(wall.0.normal.as_vec2());
                return;
            }
        }

        transform.translation += ball_movement_this_frame.extend(0.);
    }
}
