use bevy::app::App;
use bevy::prelude::*;
use bevy_simple_text_input::TextInputPlugin;

use systems::interactions::terminal_input;
use systems::layout::spawn_main_ui;

use crate::AppState;
use crate::main_ui::systems::interactions::{send_message, tab_complete};

mod components;
mod styles;
mod systems;

pub struct MainUiPlugin;

impl Plugin for MainUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TextInputPlugin)
            .add_systems(OnEnter(AppState::Start), spawn_main_ui)
            .add_systems(Update, (terminal_input, send_message, tab_complete));
    }
}