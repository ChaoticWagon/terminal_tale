use bevy::prelude::*;
use bevy::text::BreakLineOn;
use bevy_simple_text_input::TextInputBundle;

use crate::main_ui::components::{MainUi, TerminalText};
use crate::main_ui::styles::{FONT_SIZE, get_terminal_font, get_title_text_style, INPUT_STYLE, MAIN_UI_STYLE, TERMINAL_STYLE, USERNAME_STYLE};

pub fn spawn_main_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_ui(&mut commands, &asset_server);
}

pub fn despawn_main_ui(mut commands: Commands, main_ui_query: &Query<Entity, With<MainUi>>) {
    if let Ok(main_ui_entity) = main_ui_query.get_single() {
        commands.entity(main_ui_entity).despawn_recursive();
    }
}

pub fn build_main_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_ui_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_UI_STYLE,
                ..default()
            },
            MainUi {},
        ))
        .with_children(|parent| {
            // === Title ===
            parent
                .spawn(NodeBundle {
                    style: TERMINAL_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Text
                    parent.spawn((TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "", get_title_text_style(asset_server)),
                            ],
                            justify: JustifyText::Left,
                            linebreak_behavior: BreakLineOn::WordBoundary,
                        },
                        ..default()
                    }, TerminalText {}));
                });

            parent.spawn(
                NodeBundle {
                    style: INPUT_STYLE,
                    ..default()
                }).with_children(|parent| {
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new("root>", get_title_text_style(asset_server))
                            ],
                            justify: JustifyText::Left,
                            linebreak_behavior: BreakLineOn::WordBoundary,
                        },
                        style: USERNAME_STYLE,
                        ..default()
                    });

                parent.spawn((
                    NodeBundle {
                        style: INPUT_STYLE,
                        ..default()
                    },
                    TextInputBundle::default().with_text_style(
                        TextStyle {
                            font: get_terminal_font(asset_server),
                            font_size: FONT_SIZE,
                            ..default()
                        }
                    )));
            });
        })
        .id();

    main_ui_entity
}