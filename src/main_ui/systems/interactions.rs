use std::ops::Add;
use bevy::prelude::*;
use bevy_simple_text_input::TextInputSubmitEvent;

use crate::main_ui::components::{TerminalText, TerminalUser};

pub fn terminal_input(
    mut events: EventReader<TextInputSubmitEvent>,
    mut text_query: Query<
        &mut Text,
        (With<TerminalText>, Without<TerminalUser>)
    >,
    mut user_query: Query<
        &mut Text,
        (With<TerminalUser>, Without<TerminalText>)
    >,
) {
    for event in events.read() {
        if event.value.is_empty() {
            return;
        }

        // If the textbox already has text copy the Style and use that for the new text section, if not use defaults

        let username= user_query.get_single().unwrap().sections.last().unwrap().value.clone() + " ";

        for mut text in &mut text_query {
            match text.sections.clone().last_mut() {
                Some(last) => {
                    text.sections.push(TextSection {
                        value: ["\n", &username, &event.value].concat(),
                        style: last.style.clone(),
                    });
                }
                None => {
                    text.sections.push(TextSection {
                        value: event.value.clone(),
                        ..default()
                    });
                }
            }
        }
    }
}