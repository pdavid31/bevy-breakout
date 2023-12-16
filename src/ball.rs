use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::velocity::Velocity;

const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
const BALL_COLOR: Color = Color::rgb(0.94902, 0.26275, 0.20000);

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    material_bundle: MaterialMesh2dBundle<ColorMaterial>,
    ball: Ball,
    velocity: Velocity,
}

impl BallBundle {
    fn new(
        location: Vec3,
        size: Vec3,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            material_bundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(BALL_COLOR)),
                transform: Transform::from_translation(location).with_scale(size),
                ..default()
            },
            ball: Ball,
            velocity: Velocity::new(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
        }
    }
}
fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(BallBundle::new(
        BALL_STARTING_POSITION,
        BALL_SIZE,
        meshes,
        materials,
    ));
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
