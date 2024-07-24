use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

use crate::main_ui::components::MainUi;

pub fn terminal_input(
    mut events: EventReader<KeyboardInput>,
    mut text_query: Query<(&MainUi, &mut Text)>,
) {
    if events.is_empty() {
        return;
    }

    for (_, mut text) in &mut text_query {
        for event in events.read() {
            if !event.state.is_pressed() {
                continue;
            }
            
            println!("{:?}", event);

            let sec = if let Some(sec) = text.sections.last_mut() { sec } else {
                text.sections.push(
                    default()
                );
                &mut text.sections[0]
            };

            match event.key_code {
                KeyCode::Space => {
                    sec.value += " ";
                }
                KeyCode::Enter => {
                    sec.value += "\n";
                }
                KeyCode::Backspace => {
                    sec.value.pop();
                }
                _ => {}
            }

            if let Key::Character(ref s) = event.logical_key {
                println!("Logic");
                sec.value = sec.value.chars().chain(s.chars()).collect();
            }
        }
    }
}