use bevy::color::{Color, Srgba};
use bevy::ui::BackgroundColor;
use bevy::color::palettes::css::*;

pub const GRID_SIZE: u16 = 256;
pub const BOMB_COUNT:  usize = 45;
pub const CELL_COLOR :BackgroundColor = BackgroundColor(Color::Srgba(Srgba::new(0.95,0.95,0.95,1.0)));
pub const OPENED_COLOR:BackgroundColor = BackgroundColor(Color::Srgba(Srgba::new(0.7,0.7,0.7,1.0)));
pub const EXPLOEDE_BOMB_COLOR :BackgroundColor = BackgroundColor(Color::Srgba(DARK_RED));
pub const UNMARKED_BOMB_COLOR :BackgroundColor = BackgroundColor(Color::Srgba(LIMEGREEN));
pub const TEXT_COLOR :Color = Color::Srgba(BLACK);