use bevy::prelude::*;
#[derive(Component,Default)]
pub struct CellButton {
    pub opened: bool,
    pub marked: bool,
    pub hasbomb: bool,
    pub index: u16,
    pub bomb_count:u8,
}
#[derive(Component)]
pub struct Grid;

#[derive(Resource)]
pub struct BombCount{
    count:u8,
}
#[derive(Event,Debug)]
pub struct ClickCell;

#[derive(Event,Debug)]
pub struct OnClickCell{
    pub index:u16,
    pub hasbomb:bool,
    pub marked:bool,
}

#[derive(Event,Debug)]
pub struct OnClickOpenedCell{
    pub index:u16,
    pub bomb_count:u8,
}

#[derive(Event,Debug)]
pub struct ExplodeCell{
    pub index:u16,
}
#[derive(Event,Debug)]
pub struct Gameover;

#[derive(Resource,Default,Debug)]
pub struct VisibleState{
    pub state:bool,
}