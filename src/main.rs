use bevy::prelude::*;
use bevy::winit::WinitSettings;
use bevy_inspector_egui::prelude::*;
use mineswepper::cell;

// `InspectorOptions` are completely optional
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Configuration {
    name: String,
    #[inspector(min = 0.0, max = 1.0)]
    option: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        // .init_resource::<Configuration>() // `ResourceInspectorPlugin` won't initialize the resource
        // .register_type::<Configuration>() // you need to register your type to display it
        // .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
        // .add_plugins(ResourceInspectorPlugin::<Time>::default())
        .add_plugins(cell::MyPlugin)
        .run();
}
