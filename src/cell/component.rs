use bevy::prelude::*;
#[derive(Component,Default)]
pub struct CellButton {
    pub revailed: bool,
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
pub struct CheckCell{
    pub index:u16,
    pub hasbomb:bool,
    pub revaild:bool,
}
#[derive(Event,Debug)]
pub struct OnClickCell{
    pub index:u16,
}
#[derive(Event)]
pub struct OnBombCountZero{
    pub index:u16,
    pub bomb_count:u8,
}
#[derive(Event,Debug)]
pub struct OnZeroRecursive{
    pub index:u16,
}
#[derive(Event,Debug)]
pub struct Gameover;

#[derive(Resource,Default,Debug)]
pub struct VisibleState{
    pub state:bool,
}