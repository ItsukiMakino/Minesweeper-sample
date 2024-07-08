use bevy::{input::InputPlugin, prelude::*, };
use mineswepper::cell::{self, component::CellButton};
use once_cell::sync::Lazy;
use std::sync::Once;

#[test]
fn grid_spawn_test() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(cell::MyPlugin);
    app.add_plugins(InputPlugin);
    app.update();
    assert_eq!(
        app.world_mut()
            .query::<&CellButton>()
            .iter(app.world())
            .len(),
        256
    );
}
#[test]
fn test_example() {
    assert_ne!(1 + 1, 1);
}
