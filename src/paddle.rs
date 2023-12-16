use bevy::prelude::*;

use crate::{
    collision::Collider,
    wall::{BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, WALL_THICKNESS},
};

// These constants are defined in `Transform` units
// Using the default 2D camera they correspond 1:1 with screen pixels.
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;
// color
const PADDLE_COLOR: Color = Color::rgb(0.72941, 0.10588, 0.11373);

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

fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    // calculate the new horizontal paddle position based on player input
    let new_paddle_position =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_seconds();

    // update the paddle position, making sure
    // it does not cause the paddle to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL + WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}

fn setup(mut commands: Commands) {
    // calculate the y position of the paddle
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
    let paddle_location = Vec2::new(0.0, paddle_y);

    commands.spawn(PaddleBundle::new(paddle_location));
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, move_paddle);
    }
}
