use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::wall::{BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL};

const BRICK_SIZE: Vec2 = Vec2::new(100.0, 30.0);

const BRICK_COLOR: Color = Color::rgb(0.33725, 0.30196, 0.29020);

const GAP_BETWEEN_BRICKS: f32 = 5.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_PADDLE: f32 = 270.0;

#[derive(Component)]
pub struct Brick;

#[derive(Bundle)]
pub struct BrickBundle {
    sprite_bundle: SpriteBundle,
    brick: Brick,
    collider: Collider,
    body: RigidBody,
    active: ActiveEvents,
}

impl BrickBundle {
    fn new(location: Vec2) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.extend(0.0),
                    scale: BRICK_SIZE.extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: BRICK_COLOR,
                    ..default()
                },
                ..default()
            },
            brick: Brick,
            collider: Collider::cuboid(0.5, 0.5),
            body: RigidBody::Fixed,
            active: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

#[derive(Event)]
pub struct BrickDestroyedEvent;

fn setup(mut commands: Commands) {
    let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2.0 * GAP_BETWEEN_BRICKS_AND_SIDES;
    let bottom_edge_of_bricks = BOTTOM_WALL + GAP_BETWEEN_BRICKS_AND_PADDLE;
    let total_height_of_bricks = TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;

    assert!(total_width_of_bricks > 0.0);
    assert!(total_height_of_bricks > 0.0);

    // Given the space available, compute how many rows and columns of bricks we can fit
    let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_rows = (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_vertical_gaps = n_columns - 1;

    // Because we need to round the number of columns,
    // the space on the top and side of the bricks only captures
    // a lower bound, not an exact value
    let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
    let left_edge_of_bricks = center_of_bricks
        // space taken up by bricks
        - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
        // space taken up by gaps
        - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;

    // in bey, the `translation` of an entity describes the center point,
    // not its bottom-left corner
    let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.0;
    let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.0;

    // spawn bricks
    for row in 0..n_rows {
        for column in 0..n_columns {
            let brick_position = Vec2::new(
                offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
            );

            commands.spawn(BrickBundle::new(brick_position));
        }
    }
}

fn remove_brick_on_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<Entity, With<Brick>>,
    mut events: EventWriter<BrickDestroyedEvent>,
) {
    // iterate over the collision events
    for collision_event in collision_events.read() {
        // extract the entities from the event
        // in case it is of type Stopped
        if let CollisionEvent::Stopped(entity_1, entity_2, _) = collision_event {
            // loop over all entities that have the Brick component attached
            for brick in query.iter() {
                // if the brick is one of the collided entities,
                // despawn it from the game
                if brick == *entity_1 || brick == *entity_2 {
                    commands.entity(brick).despawn();
                    events.send(BrickDestroyedEvent);
                }
            }
        };
    }
}

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BrickDestroyedEvent>()
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, remove_brick_on_collision);
    }
}
