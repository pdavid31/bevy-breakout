use bevy::prelude::*;

#[derive(Component)]
pub struct Collider;

#[derive(Event)]
pub struct CollisionEvent;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {}
}
