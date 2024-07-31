use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::*;

use crate::main_ui::MainUiPlugin;
use crate::resources::script_dispatch::ScriptDispatch;

mod main_ui;
mod camera;
mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(MainUiPlugin)
        .insert_resource(
            ScriptDispatch::new().expect("Failed to load script dispatch")
        )
        .add_systems(Startup, camera::spawn_camera)
        .run();
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Start,
    Middle,
    End,
}