use bevy::prelude::*;

use crate::main_ui::components::MainUi;
use crate::main_ui::styles::{get_title_text_style, IMAGE_STYLE, MAIN_UI_STYLE, TITLE_STYLE};

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
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Hello World",
                                get_title_text_style(asset_server),
                            )],
                            justify: JustifyText::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    main_ui_entity
}