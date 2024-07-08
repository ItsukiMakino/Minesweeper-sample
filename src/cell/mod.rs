use bevy::{input::common_conditions::input_just_pressed, prelude::*};
pub mod component;
pub mod style;
pub mod system;
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, system::setup);
        app.add_systems(Update, system::click_cell);
        app.add_systems(Update, system::reset
            .run_if(input_just_pressed(KeyCode::KeyM)));
    }
}
