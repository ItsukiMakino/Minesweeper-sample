use bevy::prelude::*;
#[derive(Component)]
pub struct CellButton {
    pub revailed: bool,
    pub hasbomb: bool,
    pub index: u16,
}
impl Default for CellButton {
    fn default() -> Self {
        CellButton {
            revailed: false,
            hasbomb: false,
            index: 0,
        }
    }
}
#[derive(Component)]
pub struct Grid;