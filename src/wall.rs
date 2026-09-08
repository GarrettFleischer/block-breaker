use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Wall(pub Plane2d);

impl Wall {
    pub fn spawn(commands: &mut Commands, normal: Vec2, size: Vec2) {
        commands.spawn((
            Wall(Plane2d::new(normal)),
            Transform::from_xyz(size.x, size.y, 0.),
        ));
    }
}
