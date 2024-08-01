use bevy::{app::AppExit, input::InputPlugin};
use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy_asset_loader::prelude::*;
use rand::seq::SliceRandom;
use mineswepper::cell::{self, component::*, states::{self, *}, style::*};

#[test]
fn spawn_test() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(StatesPlugin);
    app.init_state::<states::AppState>();
    app.add_systems(Startup,setup);
    app.update();

    let mut has_bomb_count:usize =0;
    let mut binding = app.world_mut().query::<&CellButton>();
    let query = binding.iter(app.world());
    for cell in query {
        if cell.hasbomb{
            has_bomb_count +=1;
        }
    }
    assert_eq!(app.world_mut().query::<&CellButton>().iter(app.world()).len() as u16, GRID_SIZE);
    assert_eq!(has_bomb_count, BOMB_COUNT);

   
}
fn setup(
     mut commands: Commands,
     state: Res<State<AppState>>,
     mut next_state: ResMut<NextState<AppState>>,
){
    let mut rng = rand::thread_rng();
    let mut indices: Vec<u16> = (0..GRID_SIZE).collect();
    indices.shuffle(&mut rng);
    let random_indices = &indices[..BOMB_COUNT];
    for index in 0..GRID_SIZE {
        commands.spawn(CellButton{
            index,
            hasbomb:random_indices.contains(&index),
            ..Default::default()
        });
    }
    next_state.set(AppState::InGame);
}
#[test]
fn reset_test() {
     let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(cell::MyPlugin);
    app.add_plugins(StatesPlugin);
    app.insert_resource(VisibleState{state:true});
    let mut input = ButtonInput::<KeyCode>::default();
    input.press(KeyCode::KeyM);
    app.insert_resource(input);
    app.update();

    assert_eq!(app.world().resource::<VisibleState>().state,false);

    let mut binding = app.world_mut().query::<&CellButton>();
    let cells = binding.iter(app.world());
    let mut count = 0;
    for cell in cells{
        if cell.hasbomb{
            count +=1;
        }
    }
    assert_eq!(count,BOMB_COUNT);
}

#[test]
fn get_neighboring_indices_test()
{
    let grid_size = GRID_SIZE;
    let index = 0;
    let row_size = (grid_size as f64).sqrt() as u8;
    let row = index / row_size;
    let col = index % row_size;

    let mut neighbors = Vec::new();

    for y in -1..=1 {
        for x in -1..=1 {
            if y == 0 && x == 0 {
                continue;
            }
            let new_row = row as isize + y;
            let new_col = col as isize + x;

            if new_row >= 0 && new_row < row_size as isize && new_col >= 0 && new_col < row_size as isize {
                neighbors.push((new_row as u8 * row_size) + new_col as u8);
            }
        }
    }
    assert_eq!(neighbors.len(),3);
    assert_eq!(neighbors[0], 1);
    assert_eq!(neighbors[1], 16);
    assert_eq!(neighbors[2], 17);

}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
