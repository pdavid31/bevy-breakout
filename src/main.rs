mod background;
mod block;
mod camera;
mod collision;
mod paddle;
mod score;
mod wall;

use bevy::prelude::*;

use background::BackgroundPlugin;
use block::BlockPlugin;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use paddle::PaddlePlugin;
use score::ScorePlugin;
use wall::WallPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(BlockPlugin)
        // .add_plugins(CollisionPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WallPlugin)
        // .add_plugins(ScorePlugin)
        .run();
}
