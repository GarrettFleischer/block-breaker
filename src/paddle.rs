use super::level::Level;
use bevy::{color::palettes::tailwind::SKY_50, prelude::*};

const DEFAULT_PADDLE_SIZE: Vec2 = Vec2::new(200.0, 20.0);
const PADDLE_SPEED: f32 = 400.0;

#[derive(Debug, Component)]
pub struct Paddle;

#[derive(Debug, Component)]
pub struct HalfSize(pub Vec2);

impl Paddle {
    pub fn spawn(commands: &mut Commands, position: Vec2) {
        commands.spawn((
            Paddle,
            Transform::from_xyz(position.x, position.y, 0.),
            Sprite {
                custom_size: Some(DEFAULT_PADDLE_SIZE),
                color: SKY_50.into(),
                ..default()
            },
            HalfSize(DEFAULT_PADDLE_SIZE / 2.),
        ));
    }
}

pub fn controls(
    input: Res<ButtonInput<KeyCode>>,
    mut paddles: Query<(&mut Transform, &HalfSize), With<Paddle>>,
    level: Single<&Level>,
    time: Res<Time>,
) {
    for (mut transform, hs) in &mut paddles {
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= PADDLE_SPEED * time.delta_secs();
        } else if input.pressed(KeyCode::KeyD) {
            transform.translation.x += PADDLE_SPEED * time.delta_secs();
        }

        transform.translation = transform.translation.clamp(
            Vec3::new(-level.size.x / 2. + hs.0.x, transform.translation.y, 0.),
            Vec3::new(level.size.x / 2. - hs.0.x, transform.translation.y, 0.),
        );
    }
}
