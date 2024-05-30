use bevy::prelude::*;

pub const NORMAL_BTN_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

pub const BTN_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style
};

pub const IMG_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(64.0);
    style.height = Val::Px(64.0);
    style.margin = UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));
    style
};

pub const MAIN_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style
};

pub const HEADER_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(600.0);
    style.height = Val::Px(120.0);
    style
};

pub fn get_header_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font_size: 64.0,
        color: Color::BLUE,
        font: asset_server.load("fonts/amatic.ttf"),
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font_size: 32.0,
        color: Color::GREEN,
        font: asset_server.load("fonts/amatic.ttf"),
    }
}
