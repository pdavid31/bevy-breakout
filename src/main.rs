mod background;
mod ball;
mod brick;
mod camera;
mod paddle;
mod score;
mod wall;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use background::BackgroundPlugin;
use ball::BallPlugin;
use brick::BrickPlugin;
use camera::CameraPlugin;
use paddle::PaddlePlugin;
use score::ScorePlugin;
use wall::WallPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(BackgroundPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(BrickPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(WallPlugin)
        .run();
}
