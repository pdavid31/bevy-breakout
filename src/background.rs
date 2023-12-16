use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.96863, 0.95686, 0.95294);
const BACKGROUND: ClearColor = ClearColor(BACKGROUND_COLOR);

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BACKGROUND);
    }
}
