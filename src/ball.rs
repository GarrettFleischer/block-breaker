use super::paddle::Paddle;
use super::physics::HalfSize;
use super::physics::Velocity;
use super::wall::Wall;
use bevy::math::FloatOrd;
use bevy::math::bounding::{Aabb2d, RayCast2d};
use bevy::{color::palettes::tailwind::SLATE_950, prelude::*};
use std::f32::consts::{FRAC_PI_4, PI};

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
    mut commands: Commands,
    mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    walls: Query<(&Wall, &Transform), Without<Ball>>,
    aabb_colliders: Query<(Entity, &Transform, &HalfSize), Without<Ball>>,
    paddles: Query<(), With<Paddle>>,
    time: Res<Time>,
) {
    // for each ball
    for (mut transform, mut velocity) in &mut balls {
        // build iters of the walls and aabbs
        let wall_planes = walls
            .iter()
            .map(|(wall, transform)| (wall.0, transform.translation.xy()));
        let aabbs = aabb_colliders.iter().map(|(entity, transform, half_size)| {
            (entity, transform.translation.xy(), half_size.0)
        });
        // calculate basic ball info
        let ball_pos = transform.translation.xy();
        let ball_ray = Ray2d::new(ball_pos, Dir2::new(velocity.0).unwrap());
        let ball_movement_this_frame = velocity.0 * time.delta_secs();
        let ball_move_distance = ball_movement_this_frame.length();

        // bounce on the first wall hit
        if let Some(normal) = first_wall_hit(ball_ray, ball_move_distance, wall_planes) {
            velocity.0 = velocity.0.reflect(normal);
            continue;
        }

        // bounce on the paddle or bricks
        let ball_cast = RayCast2d::from_ray(ball_ray, ball_move_distance);
        if let Some((entity, origin, half_size)) = nearest_aabb_hit(&ball_cast, aabbs) {
            if paddles.get(entity).is_ok() {
                velocity.0 = paddle_bounce_velocity(ball_pos, origin, velocity.0.length());
            } else {
                let aabb = Aabb2d::new(origin, half_size); // keep half_size from nearest_aabb_hit
                // SAFETY we can unwrap because we already know we have a collision
                let brick_normal = brick_hit_normal(ball_ray, origin, aabb).unwrap();
                commands.entity(entity).despawn();
                velocity.0 = velocity.0.reflect(brick_normal);
            }
            continue;
        }

        transform.translation += ball_movement_this_frame.extend(0.);
    }
}

fn first_wall_hit(
    ray: Ray2d,
    max_distance: f32,
    walls: impl IntoIterator<Item = (Plane2d, Vec2)>,
) -> Option<Vec2> {
    walls.into_iter().find_map(|(plane, origin)| {
        let hit_distance = ray.intersect_plane(origin, plane)?;
        (hit_distance <= max_distance).then_some(plane.normal.as_vec2())
    })
}

fn nearest_aabb_hit(
    raycast: &RayCast2d,
    colliders: impl IntoIterator<Item = (Entity, Vec2, Vec2)>,
) -> Option<(Entity, Vec2, Vec2)> {
    // filter for collisions, take the shortest distance, map to the index of that collider
    colliders
        .into_iter()
        .filter_map(|(entity, origin, half_size)| {
            raycast
                .aabb_intersection_at(&Aabb2d::new(origin, half_size))
                .map(|hit_distance| (entity, origin, half_size, hit_distance))
        })
        .min_by_key(|(_, _, _, distance)| FloatOrd(*distance))
        .map(|(entity, origin, half_size, _)| (entity, origin, half_size))
}

fn paddle_bounce_velocity(ball_pos: Vec2, paddle_pos: Vec2, speed: f32) -> Vec2 {
    let angle = (ball_pos - paddle_pos).to_angle();
    let linear_angle = angle.clamp(0., PI) / PI;
    let softened_angle = FRAC_PI_4.lerp(PI - FRAC_PI_4, linear_angle);
    Vec2::from_angle(softened_angle) * speed
}

fn brick_hit_normal(ray: Ray2d, origin: Vec2, aabb: Aabb2d) -> Option<Vec2> {
    [
        (Plane2d::new(Vec2::NEG_Y), Vec2::new(origin.x, aabb.min.y)),
        (Plane2d::new(Vec2::Y), Vec2::new(origin.x, aabb.max.y)),
        (Plane2d::new(Vec2::NEG_X), Vec2::new(aabb.min.x, origin.y)),
        (Plane2d::new(Vec2::X), Vec2::new(aabb.max.x, origin.y)),
    ]
    .into_iter()
    .filter_map(|(plane, location)| {
        ray.intersect_plane(location, plane)
            .map(|hit_distance| (plane.normal.as_vec2(), hit_distance))
    })
    .min_by_key(|(_, distance)| FloatOrd(*distance))
    .map(|(normal, _)| normal)
}
