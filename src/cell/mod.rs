use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_asset_loader::loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt};
use component::VisibleState;
pub mod component;
pub mod style;
pub mod system;
pub mod states;
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VisibleState{..default()});
        app.init_state::<states::AppState>()
        .add_loading_state( LoadingState::new(states::AppState::Loading)
            .continue_to_state(states::AppState::Loaded)
            .load_collection::<states::FontAssets>(),);

        app.add_systems(OnEnter(states::AppState::Loaded), system::setup);
        
        app.add_systems(Update, system::mark_cell
            .run_if(in_state(states::AppState::InGame))
            .run_if(input_just_pressed(MouseButton::Right)));

        app.add_systems(Update, system::click_cell.run_if(in_state(states::AppState::InGame)));

        app.add_systems(Update, system::toggle_visible
            .run_if(in_state(states::AppState::InGame))
            .run_if(input_just_pressed(KeyCode::Space)))
            .observe(system::gameover)
            .observe(system::on_click_cell)
            .observe(system::on_click_opened_cell)
            .observe(system::on_explode_cell);

        app.add_systems(Update, system::reset
            .run_if(input_just_pressed(KeyCode::KeyM)));
    }
}
