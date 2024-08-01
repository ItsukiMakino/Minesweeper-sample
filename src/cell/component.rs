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
pub struct OnCellMarked{
    pub index:u16,
}

#[derive(Event,Debug)]
pub struct ExplodeCell{
    pub index:u16,
}
#[derive(Event,Debug)]
pub struct Gameover;
#[derive(Event,Debug)]
pub struct GameClear;

#[derive(Resource,Default,Debug)]
pub struct VisibleState{
    pub state:bool,
}
#[derive(Resource,Default,Debug)]
pub struct GameState{
    pub is_clear:bool,
}