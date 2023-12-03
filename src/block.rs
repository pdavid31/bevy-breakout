use bevy::prelude::*;

#[derive(Component)]
pub struct Block;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {}
}
