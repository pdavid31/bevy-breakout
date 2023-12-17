use bevy::prelude::*;

use crate::brick::BrickDestroyedEvent;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_FONT_COLOR: Color = Color::rgb(0.35686, 0.13725, 0.20000);

const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

#[derive(Resource)]
pub struct Score(usize);

impl Score {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

#[derive(Bundle)]
pub struct ScoreBundle {
    text_bundle: TextBundle,
}

impl ScoreBundle {
    fn new() -> Self {
        Self {
            text_bundle: TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: SCOREBOARD_FONT_COLOR,
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_FONT_COLOR,
                    ..default()
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            }),
        }
    }
}

fn update_score(score: Res<Score>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.0.to_string();
}

fn update_score_on_collision(
    mut brick_events: EventReader<BrickDestroyedEvent>,
    mut score: ResMut<Score>,
) {
    for _ in brick_events.read() {
        score.increment();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(ScoreBundle::new());
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, update_score_on_collision)
            .add_systems(FixedUpdate, update_score);
    }
}
