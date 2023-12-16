use bevy::prelude::*;

use crate::collision::Collider;

// x coordinates
pub const LEFT_WALL: f32 = -450.0;
pub const RIGHT_WALL: f32 = 450.0;
// y coordinates
pub const BOTTOM_WALL: f32 = -300.0;
pub const TOP_WALL: f32 = 300.0;
// render line thickness
pub const WALL_THICKNESS: f32 = 10.0;
// wall color
const WALL_COLOR: Color = Color::rgb(0.35686, 0.13725, 0.20000);

#[derive(Component)]
pub struct Wall;

pub enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            Self::Left => Vec2::new(LEFT_WALL, 0.0),
            Self::Right => Vec2::new(RIGHT_WALL, 0.0),
            Self::Bottom => Vec2::new(0.0, BOTTOM_WALL),
            Self::Top => Vec2::new(0.0, TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        // make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            Self::Left | Self::Right => Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS),
            Self::Bottom | Self::Top => Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS),
        }
    }
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    wall: Wall,
    collider: Collider,
}

impl WallBundle {
    // The constructor allows us to easily reuse logic across our wall entities
    fn new(location: WallLocation) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // we need to convert our Vec2 to Vec3 by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // the z-scale of our 2d objects must always be 1.0,
                    // or their ordering will be affected in suprising ways.
                    // See: See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            wall: Wall,
            collider: Collider,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
