use bevy::app::App;
use bevy::prelude::*;
use crate::AppState;
use systems::layout::*;
mod components;
mod styles;
mod systems;

pub struct MainUiPlugin;

impl Plugin for MainUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Start), spawn_main_ui);
    }
}