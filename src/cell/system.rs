use super::component::*;
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(create_screen_node())
        .with_children(|parent| {
            parent.spawn((create_node_parent(),Grid{}))
            .with_children(|parent| {
                let mut rng = rand::thread_rng();
                let mut indices: Vec<u16> = (0..256).collect();
                indices.shuffle(&mut rng);
                let random_indices = &indices[..40];
                for index in 0..256 {
                    parent.spawn((
                        create_button(),
                        CellButton {
                            index,
                            hasbomb:random_indices.contains(&index),
                            ..Default::default()
                        },
                    ));
                }
            });
        });
}
fn create_screen_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::srgb(255.0, 0.0, 0.0)),
        ..Default::default()
    }
}
fn create_node_parent() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Px(640.0),
            height: Val::Px(640.0),
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex(16, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(16, 1.0),
            aspect_ratio: Some(1.0),
            row_gap: Val::Px(4.0),
            column_gap: Val::Px(4.0),
            padding: UiRect::all(Val::Px(4.0)),
            // justify_content: JustifyContent::FlexStart,
            ..Default::default()
        },
        background_color: Color::srgb(0.2, 0.4, 1.).into(),
        ..Default::default()
    }
}
// fn create_row() -> NodeBundle
// {
//     NodeBundle{
//         style:Style{
//             width:Val::Px(640.0),
//             height:Val::Percent(100.0/16.0),
//             flex_direction: FlexDirection::Row,
//             justify_content: JustifyContent::SpaceEvenly,

//             margin: UiRect::all(Val::Px(2.0)),
//             ..Default::default()
//         },
//         background_color: Color::srgb(0.4, 0.4, 1.).into(),
//         ..Default::default()
//     }
// }
fn create_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            display: Display::Grid,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::WHITE),
        ..Default::default()
    }
}
fn create_cellbutton(index:u16) -> CellButton{
    CellButton{
        index,
        ..Default::default()
    }
}
#[allow(clippy::type_complexity)]
pub fn click_cell(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut CellButton),
        (Changed<Interaction>, With<CellButton>),
    >,
) {
    for (interaction, mut color, mut button) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                if !button.revailed {
                    check_cell(&mut button);
                }
                else{
                }
                *color = Color::srgb(0.1, 0.0, 0.0).into();
            }
            Interaction::Hovered => {}
            Interaction::None => {
                *color = BackgroundColor(Color::WHITE);
            }
        }
    }
}
pub fn reset(mut commands: Commands,
    cell_query: Query<Entity, With<CellButton>>,
    parent: Query<Entity,With<Grid>>,
)
{
    for entity in cell_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for p in parent.iter(){

        for index in 0..256 {
            let id = commands.spawn((
                create_button(),
                CellButton {
                    index,
                    ..Default::default()
                },
            )).id();
            commands.entity(p).push_children(&[id]);
        }
        
    }
}
fn check_cell(button :&mut Mut<CellButton>,)
{
    button.revailed = true;
    if button.hasbomb
    {
        println!("gameover");
        return;
    }
    let neighbor = get_neighboring_indices(button.index);

    println!("{}, {}, {:?}",button.revailed ,button.index,neighbor);
}
fn get_neighboring_indices(index:u16) ->Vec<u16>
{
    let grid_size = 256;
    let row_size = (grid_size as f64).sqrt() as u16;
    let row = index / row_size;
    let col = index % row_size;

    let mut neighbors:Vec<u16> = Vec::new();

    for y in -1..=1 {
        for x in -1..=1 {
            if y == 0 && x == 0 {
                continue;
            }
            let new_row = row as isize + y;
            let new_col = col as isize + x;

            if new_row >= 0 && new_row < row_size as isize && new_col >= 0 && new_col < row_size as isize {
                neighbors.push((new_row as u16 * row_size) + new_col as u16);
            }
        }
    }
    neighbors
}