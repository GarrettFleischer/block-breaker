use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}

#[derive(Debug, Component)]
pub struct HalfSize(pub Vec2);
