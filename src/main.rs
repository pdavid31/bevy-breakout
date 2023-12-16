mod background;
mod ball;
mod brick;
mod camera;
mod collision;
mod paddle;
mod score;
mod velocity;
mod wall;

use bevy::prelude::*;

use background::BackgroundPlugin;
use ball::BallPlugin;
use brick::BrickPlugin;
use camera::CameraPlugin;
use collision::CollisionPlugin;
use paddle::PaddlePlugin;
use score::ScorePlugin;
use velocity::VelocityPlugin;
use wall::WallPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BackgroundPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(BrickPlugin)
        .add_plugins(CameraPlugin)
        // .add_plugins(CollisionPlugin)
        .add_plugins(PaddlePlugin)
        // .add_plugins(ScorePlugin)
        .add_plugins(VelocityPlugin)
        .add_plugins(WallPlugin)
        .run();
}
