use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_simple_text_input::{TextInputCursorPos, TextInputCursorTimer, TextInputInactive, TextInputSettings, TextInputSubmitEvent, TextInputValue};

use crate::main_ui::components::{TerminalPrompt, TerminalText, TerminalUser};
use crate::resources::script_dispatch::{MessageMode, ScriptDispatch};

pub fn terminal_input(
    mut events: EventReader<TextInputSubmitEvent>,
    mut text_query: Query<
        &mut Text,
        (With<TerminalText>, Without<TerminalUser>, Without<TerminalPrompt>)
    >,
    mut user_query: Query<
        &mut Text,
        (With<TerminalUser>, Without<TerminalText>, Without<TerminalPrompt>)
    >,
    mut prompt_query: Query<
        &mut Text,
        (With<TerminalPrompt>, Without<TerminalUser>, Without<TerminalText>)
    >,
    mut scripts: ResMut<ScriptDispatch>,
) {
    for event in events.read() {
        if event.value.is_empty() {
            return;
        }


        // get the phase and check if the value is a command
        let Some(phase) = scripts.phases.front_mut() else { return };
        let Some(expected_command) = phase.commands.pop_front() else { return };

        if expected_command.command != event.value {
            phase.commands.push_front(expected_command);
            return;
        }

        // pop the command from the phase

        let Some(user_text) = user_query.iter().last() else { return };
        let Some(last_username) = user_text.sections.last() else { return };
        let last_username = last_username.value.clone();
        let username = expected_command.username.clone();
        
        let Some(next_command) = phase.commands.front() else { return };
        for mut prompt in &mut prompt_query {
            if let Some(last) = prompt.sections.clone().last_mut() {
                prompt.sections.pop();
                prompt.sections.push(TextSection {
                    value: next_command.command.clone(),
                    style: last.style.clone(),
                });
            }
        }

        // for each message in the command add to the value vec

        for mut terminal in &mut text_query {
            match terminal.sections.clone().last_mut() {
                Some(last) => {
                    terminal.sections.push(TextSection {
                        value: ["\n", &last_username, " ", &event.value].concat(),
                        style: last.style.clone(),
                    });

                    // set the username
                    user_query.iter_mut().for_each(|mut user| {
                        user.sections = vec![TextSection {
                            value: username.clone() + ">",
                            style: last.style.clone(),
                        }];
                    });

                    for message in &expected_command.messages {
                        scripts.message_queue.push_back((
                            ["\n", &message.message].concat(),
                            message.mode,
                            Timer::from_seconds(message.delay, TimerMode::Once),
                        ));
                    }
                }
                None => {
                    terminal.sections.push(TextSection {
                        value: event.value.clone(),
                        ..default()
                    });
                }
            }
        }
    }
}

pub fn send_message(
    time: Res<Time>,
    mut script_dispatch: ResMut<ScriptDispatch>,
    mut text_query: Query<&mut Text, With<TerminalText>>,
) {
    if script_dispatch.message_queue.is_empty() {
        return;
    }

    for message in &mut script_dispatch.message_queue {
        message.2.tick(time.delta());
    }

    let Some(front) = script_dispatch.message_queue.pop_front() else { return };

    if front.2.finished() {
        for mut terminal in &mut text_query {
            match terminal.sections.clone().last_mut() {
                Some(last) => {
                    match front.1 {
                        MessageMode::Print => {
                            terminal.sections.push(TextSection {
                                value: front.0.clone(),
                                style: last.style.clone(),
                            });
                        }
                        MessageMode::Replace => {
                            terminal.sections.pop();
                            terminal.sections.push(TextSection {
                                value: front.0.clone(),
                                style: last.style.clone(),
                            });
                        }
                    }
                }
                None => {
                    terminal.sections.push(TextSection {
                        value: front.0.clone(),
                        ..default()
                    });
                }
            }
        }
        return;
    }

    script_dispatch.message_queue.push_front(front);
}

pub fn tab_complete(
    mut events: EventReader<KeyboardInput>,
    mut script_dispatch: ResMut<ScriptDispatch>,
    mut text_input_query: Query<(
        Entity,
        &TextInputSettings,
        &TextInputInactive,
        &mut TextInputValue,
        &mut TextInputCursorPos,
        &mut TextInputCursorTimer,
    )>,
) {
    if events.is_empty() {
        return;
    }

    for (input_entity, settings, inactive, mut text_input, mut cursor_pos, mut cursor_timer) in
        &mut text_input_query
    {
        if inactive.0 {
            continue;
        }

        let Some(phase) = script_dispatch.phases.front_mut() else { return };
        let Some(expected_command) = phase.commands.front() else { return };

        for event in events.read() {
            if !event.state.is_pressed() {
                continue;
            };

            if event.key_code == KeyCode::Tab {

                if let Some(last_char) = text_input.0.chars().last() {
                    if last_char.is_whitespace() {
                        return;
                    }
                }

                let command_words = &expected_command.command.split_whitespace().collect::<Vec<&str>>();
                let last_word = text_input.0.split_whitespace().last().unwrap_or("");

                for word in command_words {
                    if word.starts_with(last_word) {
                        let mut new_text = text_input.0.split_whitespace().collect::<Vec<&str>>();
                        new_text.pop();
                        new_text.push(word);
                        text_input.0 = new_text.join(" ");
                        cursor_pos.0 = text_input.0.len();
                        break;
                    }
                }


            }
        }
    }
}