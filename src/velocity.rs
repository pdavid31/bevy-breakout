pub use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

impl Velocity {
    pub fn new(velocity: Vec2) -> Self {
        Self(velocity)
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, apply_velocity);
    }
}
