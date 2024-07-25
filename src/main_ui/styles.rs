use bevy::prelude::*;

pub const MAIN_UI_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::FlexStart;
    style.align_items = AlignItems::Stretch;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    // style.row_gap = Val::Px(8.0);
    // style.column_gap = Val::Px(8.0);
    style
};

pub const TERMINAL_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Start;
    style.align_items = AlignItems::FlexStart;
    style.flex_wrap = FlexWrap::Wrap;
    style.width = Val::Auto;
    style.height = Val::Auto;
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};
pub const INPUT_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Start;
    style.align_items = AlignItems::FlexStart;
    style.width = Val::Auto;
    style.height = Val::Px(50.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

pub const USERNAME_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Start;
    style.align_items = AlignItems::FlexStart;
    style.width = Val::Auto;
    style.height = Val::Px(50.0);
    style.row_gap = Val::Px(2.0);
    style.column_gap = Val::Px(8.0);
    style
};



pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/1985-ibm-pc-vga/PxPlus_IBM_VGA8.ttf"),
        font_size: 14.0,
        color: Color::WHITE
    }
}

pub fn get_terminal_font(asset_server: &Res<AssetServer>) -> Handle<Font> {
    asset_server.load("fonts/1985-ibm-pc-vga/PxPlus_IBM_VGA8.ttf")
}

pub const FONT_SIZE: f32 = 14.0;
pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/1971-ibm-3278/3270-Regular.ttf"),
        font_size: 32.0,
        color: Color::WHITE
    }
}