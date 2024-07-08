use bevy::prelude::*;
#[derive(Component,Default)]
pub struct CellButton {
    pub revailed: bool,
    pub hasbomb: bool,
    pub index: u16,
}
#[derive(Component)]
pub struct Grid;