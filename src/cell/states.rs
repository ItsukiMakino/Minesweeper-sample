use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState{
    #[default]
    Loading,
    Loaded,
    InGame,
    GameClear,
    GameOver,
}
#[derive(AssetCollection, Resource,Clone)]
pub struct FontAssets {
    #[asset(path = "fonts/NotoSansCJKjp-Bold.otf")]
    pub font: Handle<Font>,
}