use bevy::prelude::*;
use mineswepper::cell::{self, component::CellButton};
use once_cell::sync::Lazy;
use std::sync::Once;

static INIT: Lazy<Once> = Lazy::new(|| Once::new());

fn setup(){
    todo!();    
}

fn initialize() {
    INIT.call_once(|| {
        setup();
    });
}
#[test]
fn grid_spawn_test()
{
    initialize();
    let mut app =  App::new();
    app.add_plugins(MinimalPlugins);  
    app.add_plugins(cell::MyPlugin);
    app.update();

    assert_eq!(app.world_mut().query::<&CellButton>().iter(app.world()).len(), 256);
}
#[test]
fn test_example()
{
    initialize();
    assert_ne!(1+1,1);
}