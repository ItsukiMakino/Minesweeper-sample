use bevy::{color::palettes::css::LIME, prelude::*};


pub fn setup(
    mut commands:Commands,)
{
    commands.spawn(Camera2dBundle::default());
    commands.spawn(create_node())
        .with_children(|parent|{
            parent.spawn(create_node_parent())
            .with_children(|parent|{
                for _ in 0..256{
                    parent.spawn(create_button());
                }
           });       
        });
        
}
fn create_node() -> NodeBundle
{
    NodeBundle{
        style:Style{
            width:Val::Percent(100.0),
            height:Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center, 
            ..Default::default()
        },
        background_color: BackgroundColor(Color::srgb(255.0,0.0,0.0,)),
        ..Default::default()    
    }
}
fn create_node_parent() ->NodeBundle
{
    NodeBundle{
        style:Style{
            width:Val::Px(640.0),
            height:Val::Px(640.0),
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex(16, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(16, 1.0),
            aspect_ratio: Some(1.0),
            row_gap: Val::Px(4.0),
            column_gap: Val::Px(4.0),
            padding:UiRect::all(Val::Px(4.0)),
            // justify_content: JustifyContent::FlexStart, 
            ..Default::default()
        },
        background_color: Color::srgb(0.2, 0.4, 1.).into(),
        ..Default::default()
    }
}
fn create_row() -> NodeBundle
{
    NodeBundle{
        style:Style{
            width:Val::Px(640.0),
            height:Val::Percent(100.0/16.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,

            margin: UiRect::all(Val::Px(2.0)),
            ..Default::default()
        },
        background_color: Color::srgb(0.4, 0.4, 1.).into(),
        ..Default::default()
    }
}
fn create_button() ->ButtonBundle
{
    ButtonBundle{
        style:Style{
            display: Display::Grid,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::WHITE),
        ..Default::default()
    }}