use bevy::prelude::*;
use bevy::text::BreakLineOn;
use bevy_simple_text_input::TextInputBundle;

use crate::main_ui::components::MainUi;
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
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "cargo build --release
warning: terminal_tale v0.1.0 (/Users/nathanbonano/RustroverProjects/terminal_tale) ignoring invalid dependency `wasm-server-runner` which is missing a lib target
   Compiling proc-macro2 v1.0.86
   Compiling unicode-ident v1.0.12
   Compiling libc v0.2.155
   Compiling cfg-if v1.0.0
   Compiling serde v1.0.204
   Compiling autocfg v1.3.0
   Compiling once_cell v1.19.0
   Compiling version_check v0.9.4
   Compiling pin-project-lite v0.2.14
   Compiling log v0.4.22
   Compiling zerocopy v0.7.35
   Compiling equivalent v1.0.1
   Compiling hashbrown v0.14.5
   Compiling toml_datetime v0.6.6
   Compiling winnow v0.6.15
   Compiling thiserror v1.0.63
   Compiling crossbeam-utils v0.8.20
   Compiling allocator-api2 v0.2.18
   Compiling ahash v0.8.11
   Compiling smallvec v1.13.2
   Compiling arrayvec v0.7.4
   Compiling tracing-core v0.1.32
   Compiling parking v2.2.0
   Compiling ppv-lite86 v0.2.17
   Compiling futures-core v0.3.30
   Compiling thread_local v1.1.8
   Compiling quote v1.0.36
   Compiling core-foundation-sys v0.8.6
   Compiling futures-io v0.3.30
   Compiling fastrand v2.1.0
   Compiling getrandom v0.2.15
   Compiling smol_str v0.2.2
   Compiling concurrent-queue v2.5.0
   Compiling uuid v1.10.0
   Compiling futures-lite v2.3.0
   Compiling syn v2.0.72
   Compiling indexmap v2.2.6
   Compiling event-listener v5.3.1",
                                get_title_text_style(asset_server),
                            )],
                            justify: JustifyText::Left,
                            linebreak_behavior: BreakLineOn::WordBoundary,
                        },
                        ..default()
                    });
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