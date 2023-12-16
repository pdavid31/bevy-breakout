use bevy::prelude::*;

#[derive(Component)]
pub struct Brick;

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {}
}
