use std::process::Child;

use super::component::*;
use super::states::{AppState, FontAssets};
use bevy::ecs::{entity, event, query};
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::seq::SliceRandom;
use super::style::*;

pub fn setup(
    mut commands: Commands,
    font:Res<FontAssets>,
    mut visible: ResMut<VisibleState>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
)
{
    visible.state = false;
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(create_screen_node())
        .with_children(|parent| {
            parent.spawn((create_node_parent(),Grid{}))
            .with_children(|parent| {
                let mut rng = rand::thread_rng();
                let mut indices: Vec<u16> = (0..GRID_SIZE).collect();
                indices.shuffle(&mut rng);
                let random_indices = &indices[..BOMB_COUNT];
                for index in 0..GRID_SIZE {
                    let font_clone = font.clone();
                    parent.spawn((
                        create_button(),
                        create_cellbutton(index,random_indices),
                    )).with_children(move |parent|{
                        parent.spawn(create_text(font_clone));
                    }); 
                }
            });
               
        });
        if *state.get() == AppState::Loaded{
            next_state.set(AppState::InGame);
        }
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
fn create_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            display: Display::Grid,
            // margin: UiRect::all(Val::Auto), 
            justify_content:JustifyContent::Center,
            align_items:AlignItems::Center,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::WHITE),
        ..Default::default()
    }
}
fn create_cellbutton(index:u16 , random_indices:&[u16]) -> CellButton{
    CellButton{
        index,
        hasbomb:random_indices.contains(&index),
        ..Default::default()
    }
}
fn create_text(asset:FontAssets) -> TextBundle{
    TextBundle{
        visibility:Visibility::Hidden,
        text:Text::from_section(
           "",
       TextStyle {
            font:asset.font,
            font_size: 640.0/16.0*0.8,
            color: Color::BLACK,
            ..default()
         })
        .with_justify(JustifyText::Center),
        style:Style{
            ..Default::default()
        },
        ..default()
    }
    
    
}
// 右クリックでマーク
pub fn mark_cell(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Node, &GlobalTransform, Mut<CellButton>, &Children)>,
    mut text_query: Query<(Mut<Text>, Mut<Visibility>)>,
)
{
    if let Some(position) = q_windows.single().cursor_position() {
        for (node, global_transform, mut cell_button,children) in query.iter_mut() {
            let transform = global_transform.compute_matrix();
            let size = node.size();
            let min = transform.transform_point3(Vec3::new(-size.x / 2.0, -size.y / 2.0, 0.0));
            let max = transform.transform_point3(Vec3::new(size.x / 2.0, size.y / 2.0, 0.0));
            
            if (min.x..max.x).contains(&position.x) && (min.y..max.y).contains(&position.y) && !cell_button.opened {
                cell_button.marked = !cell_button.marked;
                if cell_button.marked {
                    for child in children{
                        if let Ok((mut text, mut visibility)) = text_query.get_mut(*child){
                            text.sections[0].value = "M".to_string();
                            *visibility = Visibility::Visible;
                        }
                    }
                }else {
                    for child in children{
                        if let Ok((mut text, mut visibility)) = text_query.get_mut(*child){
                            text.sections[0].value = "".to_string();
                            *visibility = Visibility::Hidden;
                        }
                    }
                }
            }
        }
    } 
}
#[allow(clippy::type_complexity)]
pub fn click_cell(
    mut query: Query<
        (&Interaction,Mut<CellButton>,),
        (Changed<Interaction>, With<CellButton>,),
    >,
    mut commands: Commands,
) {
    for (interaction,mut button,) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                if button.marked{
                    return;
                }
                if !button.opened
                {
                    button.opened = true;
                    commands.trigger(OnClickCell{
                        index:button.index,
                        hasbomb:button.hasbomb,
                        marked:button.marked,
                    });   
                    return;
                }
                commands.trigger(OnClickOpenedCell{
                    index:button.index,
                    bomb_count:button.bomb_count,
                });  
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn gameover(_trigger: Trigger<Gameover>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
)
{
    if *state.get() == AppState::InGame {
        next_state.set(AppState::Gameover);
    }
    println!("GameOver!!");
}
#[allow(clippy::type_complexity)]
pub fn reset(mut commands: Commands,
    cell_query: Query<Entity, With<CellButton>>,
    font:Res<FontAssets>,
    parent: Query<Entity,With<Grid>>,
    mut visible: ResMut<VisibleState>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
)
{
    if *state.get() == AppState::Gameover{
        next_state.set(AppState::InGame);
    }
    visible.state = false;
    
    for entity in cell_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for p in parent.iter(){
        let mut rng = rand::thread_rng();
        let mut indices: Vec<u16> = (0..GRID_SIZE).collect();
        indices.shuffle(&mut rng);
        let random_indices = &indices[..BOMB_COUNT];

        for index in 0..GRID_SIZE {
            let font_clone = font.clone();
            let id = commands.spawn((
                create_button(),
                create_cellbutton(index,random_indices),
            )).with_children(move |parent|{
                parent.spawn(create_text(font_clone));
            }).id();
            commands.entity(p).push_children(&[id]);
        }
    }
}
#[allow(clippy::type_complexity)]
pub fn on_click_cell(
    trigger: Trigger<OnClickCell>,
    mut commands: Commands,
    mut neighbor_query: Query<(Mut<CellButton>,&Children),>,
    mut text_query: Query<(Mut<Text>, Mut<Visibility>)>,
    mut count :Local<u16>,
){
    if trigger.event().hasbomb && !trigger.event().marked{
        commands.trigger(Gameover);
        return;
    }
    *count = 0;
    let mut bomb_count =0;
    let neighbors = get_neighboring_indices(trigger.event().index);
      
    
    for &neighbor_index in &neighbors {
        for (button,_) in &mut neighbor_query{
            if button.index == neighbor_index{
                if button.hasbomb
                {
                    bomb_count +=1;
                }
            }
        }
        if bomb_count != 0{
            for (mut button,children) in &mut neighbor_query{
                if button.index == trigger.event().index && !button.marked{
                    button.bomb_count = bomb_count;
                    for child in children.iter() {
                        if let Ok((mut text, mut vis)) = text_query.get_mut(*child) {
                            text.sections[0].value = bomb_count.to_string();
                            *vis = Visibility::Visible;
                        }
                    }
                }
               
            }
        }
    }
    for (button,_) in &mut neighbor_query{
        if button.opened{
            *count+=1;
        }
    }
    println!("{:?}",*count);
    if *count == GRID_SIZE - (BOMB_COUNT as u16)
    {
        println!("Clear");
        return;
    }
    if bomb_count == 0{
        commands.trigger(ExplodeCell{
            index:trigger.event().index,
        });
    }
}
#[allow(clippy::type_complexity)]
pub fn on_explode_cell(
    trigger: Trigger<ExplodeCell>,
    mut commands: Commands,
    mut neighbor_query: Query<(Mut<CellButton>,&Children)>,
) {
    let mut neighbors: Vec<u16> = get_neighboring_indices(trigger.event().index);
    while let Some(index) = neighbors.pop() {
        if let Some((mut button, _)) = neighbor_query.iter_mut().find_map(|(button, children)| {
            if button.index == index && !button.opened && !button.marked {
                Some((button,children))
            } else {
                None
            }
        }) {
            button.opened = true;
            commands.trigger(OnClickCell{
                index:button.index,
                hasbomb:button.hasbomb,
                marked:button.marked,
            });
            if !neighbors.contains(&button.index) {
                neighbors.push(button.index);
            }       
        }
        
    }
}
pub fn on_click_opened_cell(
    trigger: Trigger<OnClickOpenedCell>,
    mut commands: Commands,
    mut neighbor_query: Query<(Mut<CellButton>,&Children),>,
)
{
    let mut marked_count = 0;
    let neighbors = get_neighboring_indices(trigger.event().index);
    for &neighbor_index in &neighbors {
        for (button,_) in &mut neighbor_query{
            if button.index == neighbor_index && button.marked {
                marked_count += 1;
            }
        }
    }
    if trigger.event().bomb_count == marked_count{
        for &neighbor_index in &neighbors {
            for (mut button,_) in &mut neighbor_query{
                if button.index == neighbor_index && !button.marked {
                    button.opened = true;
                    commands.trigger(OnClickCell{
                        index: button.index,
                        hasbomb:button.hasbomb,
                        marked:button.marked,
                    })
                }
            }
        }
    }
    else{
    }
}

fn get_neighboring_indices(index:u16) ->Vec<u16>
{
    let grid_size = GRID_SIZE;
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
pub fn toggle_visible(
    mut query: Query<
            (Mut<CellButton>,Mut<BackgroundColor>),
        >,
    mut visible: ResMut<VisibleState>,
)
{
    for (button, mut color) in query.iter_mut() {
        if !visible.state {
            *color = if button.hasbomb {
                Color::srgb(0.2, 0.2, 0.4).into()
            } else {
                Color::srgb(1.0, 1.0, 1.0).into()
            };
        } else {
            *color = Color::srgb(1.0, 1.0, 1.0).into();
        }
    }
    visible.state = !visible.state;
    
}