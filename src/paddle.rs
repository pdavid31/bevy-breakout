use bevy::prelude::*;

use crate::{collision::Collider, wall::Wall};

// These constants are defined in `Transform` units
// Using the default 2D camera they correspond 1:1 with screen pixels.
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;
// color
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

#[derive(Component)]
pub struct Paddle;

#[derive(Bundle)]
pub struct PaddleBundle {
    sprite_bundle: SpriteBundle,
    paddle: Paddle,
    collider: Collider,
}

impl PaddleBundle {
    fn new(location: Vec2) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.extend(0.0),
                    scale: PADDLE_SIZE,
                    ..default()
                },
                sprite: Sprite {
                    color: PADDLE_COLOR,
                    ..default()
                },
                ..default()
            },
            paddle: Paddle,
            collider: Collider,
        }
    }
}

fn setup(mut commands: Commands, walls_query: Query<&Transform, With<Wall>>) {
    // get the wall with the lowest y
    let lowest_wall_opt = walls_query.iter().min_by(|a, b| {
        a.translation
            .y
            // f32 and f64 don't implement std::cmp::Ord
            // since these types can also be NaN.
            // thus, we use the partial_cmp here and fall
            // back to Less if the Option is None
            .partial_cmp(&b.translation.y)
            .unwrap_or(std::cmp::Ordering::Less)
    });

    // if we find a wall, we spawn the paddle above that
    // the distance is given by GAP_BETWEEN_PADDLE_AND_FLOOR
    if let Some(lowest_wall) = lowest_wall_opt {
        let paddle_y = lowest_wall.translation.y + GAP_BETWEEN_PADDLE_AND_FLOOR;
        let paddle_location = Vec2::new(0.0, paddle_y);

        commands.spawn(PaddleBundle::new(paddle_location));
    } else {
        panic!("no wall found when trying to spawn the paddle");
    }
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
