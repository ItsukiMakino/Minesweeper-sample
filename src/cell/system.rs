use std::cell;

use bevy::color::palettes::css::*;
use bevy::color::{Color, Srgba};
use super::component::*;
use super::states::{AppState, FontAssets};
use bevy::ecs::{entity, event, query};
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
    println!("current state is Loaded");
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
            println!("change state to InGame");
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
        background_color:BackgroundColor(Color::BLACK),
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
        background_color: BackgroundColor(Color::BLACK),
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
        background_color: CELL_COLOR,
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
    mut commands: Commands,
    font:Res<FontAssets>, 
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
               
                commands.trigger(OnCellMarked{index:cell_button.index});
                if cell_button.marked {
                    for child in children{
                        if let Ok((mut text, mut visibility)) = text_query.get_mut(*child){
                            text.sections[0].value = "M".to_string();
                            text.sections[0].style = TextStyle{
                                color:Color::BLACK,
                                font:font.font.clone(),
                                    ..default()
                            };
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
pub fn on_cell_marked(
    trigger: Trigger<OnCellMarked>,
    mut query: Query<(Mut<CellButton>, &Children)>,
){
    let mut bomb_count = 0;
    let neighbor = get_neighboring_indices(trigger.event().index);
    for &neighbor_index in &neighbor{
        for (button, _children) in &mut query{
            if button.index == neighbor_index && button.hasbomb{
                bomb_count +=1;
            }
        }
    }
    for (mut button, _children) in &mut query{
        if button.index == trigger.event().index && bomb_count !=0 {
            button.bomb_count = bomb_count;
        }
    }
}
#[allow(clippy::type_complexity)]
pub fn click_cell(
    mut query: Query<
        (&Interaction,Mut<CellButton>,),
        (Changed<Interaction>, With<CellButton>,),>,
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

pub fn gameover(
    mut query: Query<(Mut<CellButton>,Mut<BackgroundColor>,&Children),>,
    mut text_query: Query<(Mut<Text>, Mut<Visibility>)>,
    font:Res<FontAssets>, 
)
{
    for (button, mut bg,children ) in &mut query{
        if button.hasbomb && !button.opened{
            for child in children.iter(){
                if let Ok((mut text, mut vis)) = text_query.get_mut(*child) {
                    *vis = Visibility::Visible;
                    *bg = OPENED_COLOR;
                    text.sections[0].value = "M".to_string();
                    text.sections[0].style = TextStyle{
                        color:Color::BLACK,
                        font:font.font.clone(),
                            ..default()
                    };
                }

            }
        }
        if !button.hasbomb && button.marked {
            for child in children.iter(){
                if let Ok((mut text, mut vis)) = text_query.get_mut(*child) {
                    *vis = Visibility::Visible;
                    *bg = UNMARKED_BOMB_COLOR;
                    text.sections[0].value = if button.bomb_count != 0 {
                        button.bomb_count.to_string()
                    }else{
                        "".to_string()
                    };
                    text.sections[0].style = TextStyle{
                        color:Color::BLACK,
                        font:font.font.clone(),
                            ..default()
                    };
                }

            }
        }
    }
}
pub fn gameclear(
    mut query: Query<(Mut<CellButton>,Mut<BackgroundColor>,&Children),>,
    mut text_query: Query<(Mut<Text>, Mut<Visibility>)>,
    font:Res<FontAssets>, 
){
    for (button, _bg,children ) in &mut query{
        if button.hasbomb {
            for child in children.iter(){
                if let Ok((mut text, mut vis)) = text_query.get_mut(*child) {
                    *vis = Visibility::Visible;
                    text.sections[0].value = "M".to_string();
                    text.sections[0].style = TextStyle{
                        color:Color::BLACK,
                        font:font.font.clone(),
                            ..default()
                    };
                   
                }

            }
        }
    }
   println!("Clear")
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
    match *state.get() {
        AppState::GameOver => next_state.set(AppState::InGame),
        AppState::GameClear => next_state.set(AppState::InGame),
        _ =>{}
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
    mut neighbor_query: Query<(Mut<CellButton>,Mut<BackgroundColor>,&Children),>,
    mut text_query: Query<(Mut<Text>, Mut<Visibility>)>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    font:Res<FontAssets>,   
){
    if trigger.event().hasbomb && !trigger.event().marked{
        if *state.get() == AppState::InGame {
            for (button,mut bg,children) in &mut neighbor_query{
                if button.index == trigger.event().index {
                    for child in children.iter(){
                        if let Ok((mut text, mut vis)) = text_query.get_mut(*child)
                        {
                            *bg = EXPLOEDE_BOMB_COLOR;
                            *vis = Visibility::Visible;
                            text.sections[0].value = "M".to_string();
                            text.sections[0].style = TextStyle{
                                color:Color::BLACK,
                                font:font.font.clone(),
                                    ..default()
                            };
                        }
                    }
                }
               
            }
            next_state.set(AppState::GameOver);
        }
        return;
    }
    let mut count = 0;
    let mut bomb_count =0;
    let neighbors = get_neighboring_indices(trigger.event().index);
      
    
    for &neighbor_index in &neighbors {
        for (button,_,_) in &mut neighbor_query{
            if button.index == neighbor_index{
                if button.hasbomb
                {
                    bomb_count +=1;
                }
            }
        }
        if bomb_count != 0{
            for (mut button,mut bg,children) in &mut neighbor_query{
                if button.index == trigger.event().index && !button.marked{
                    button.bomb_count = bomb_count;
                    for child in children.iter() {
                        if let Ok((mut text, mut vis)) = text_query.get_mut(*child) {
                            text.sections[0].value = bomb_count.to_string();
                            text.sections[0].style = TextStyle{
                                color:get_textcolor(bomb_count),
                                font:font.font.clone(),
                                    ..default()
                            };
                            *bg = OPENED_COLOR;
                            *vis = Visibility::Visible;
                        }
                    }
                }
               
            }
        }
    }
    for (button,_,_) in &mut neighbor_query{
        if button.opened{
            count+=1;
        }
    }
    if count == GRID_SIZE - (BOMB_COUNT as u16)
    {
        match *state.get(){
            AppState::InGame => {
                next_state.set(AppState::GameClear);
            },
            _ =>{}
        }
        return;
    }
    if bomb_count == 0{
        for (mut button,mut bg,_) in &mut neighbor_query{
            if button.index == trigger.event().index && !button.marked{
                button.bomb_count = bomb_count;
                *bg = OPENED_COLOR;
            }
           
        }
        commands.trigger(ExplodeCell{
            index:trigger.event().index,
        });
    }
}
#[allow(clippy::type_complexity)]
pub fn on_explode_cell(
    trigger: Trigger<ExplodeCell>,
    mut commands: Commands,
    mut neighbor_query: Query<(Mut<CellButton>,Mut<BackgroundColor>,&Children)>,
) {
    
    let mut neighbors: Vec<u16> = get_neighboring_indices(trigger.event().index);
    while let Some(index) = neighbors.pop() {
        if let Some((mut button,mut bg, _)) = neighbor_query.iter_mut().find_map(|(button,bg, children)| {
            if button.index == index && !button.opened && !button.marked {
                Some((button,bg,children))
            } else {
                None
            }
        }) {
            button.opened = true;
            *bg = OPENED_COLOR;
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
fn get_textcolor(bombcount:u8)-> Color
{
    match bombcount {
        1 =>Color::Srgba(BLACK),
        2 =>Color::Srgba(MEDIUM_SEA_GREEN),
        3 =>Color::Srgba(ORANGE_RED),
        4 =>Color::Srgba(PURPLE),
        5 =>Color::Srgba(ORANGE),
        6 =>Color::Srgba(MEDIUM_SEA_GREEN),
        7 =>Color::Srgba(MEDIUM_VIOLET_RED),
        8 =>Color::Srgba(GREY),
        _ =>Color::BLACK,
    }
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
           if button.hasbomb && !button.opened{
               *color =  Color::srgb(0.2, 0.2, 0.4).into()
           }
        } else {
            if !button.opened{
                *color =  Color::WHITE.into()
            }
        }
    }
    visible.state = !visible.state;
    
}