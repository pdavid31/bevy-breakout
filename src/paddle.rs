use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::wall::BOTTOM_WALL;

// These constants are defined in `Transform` units
// Using the default 2D camera they correspond 1:1 with screen pixels.
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_SPEED: f32 = 500.0;
// color
const PADDLE_COLOR: Color = Color::rgb(0.72941, 0.10588, 0.11373);

#[derive(Component)]
pub struct Paddle;

#[derive(Bundle)]
pub struct PaddleBundle {
    sprite_bundle: SpriteBundle,
    paddle: Paddle,
    controller: KinematicCharacterController,
    collider: Collider,
    velocity: Velocity,
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
            controller: KinematicCharacterController::default(),
            paddle: Paddle,
            collider: Collider::cuboid(PADDLE_SIZE.x / 256.0, PADDLE_SIZE.y / 32.0),
            velocity: Velocity {
                linvel: Vec2::new(0.0, 0.0),
                ..default()
            },
        }
    }
}

fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut KinematicCharacterController, With<Paddle>>,
    time: Res<Time>,
) {
    let mut controller = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    // construct the movement vector
    let to_move = Vec2::new(direction * PADDLE_SPEED * time.delta_seconds(), 0.0);

    // apply the new movement
    controller.translation = Some(to_move);
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
