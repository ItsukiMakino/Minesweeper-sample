use bevy::{ecs::schedule::SystemTypeSet, input::common_conditions::input_just_pressed, prelude::*};
use component::VisibleState;
pub mod component;
pub mod style;
pub mod system;
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VisibleState{..default()});
        app.add_systems(Startup, system::setup);
        app.add_systems(Update, system::click_cell);
        app.add_systems(Update, system::toggle_visible
            .run_if(input_just_pressed(KeyCode::Space)));
        app.add_systems(Update, system::reset
            .run_if(input_just_pressed(KeyCode::KeyM)))
            .observe(system::gameover)
            .observe(system::on_click_cell)
            .observe(system::on_zero_recursive);
    }
}
