mod background;
mod brick;
mod camera;
mod collision;
mod paddle;
mod score;
mod wall;

use bevy::prelude::*;

use background::BackgroundPlugin;
use brick::BrickPlugin;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use paddle::PaddlePlugin;
use score::ScorePlugin;
use wall::WallPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BrickPlugin)
        // .add_plugins(CollisionPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WallPlugin)
        // .add_plugins(ScorePlugin)
        .run();
}
