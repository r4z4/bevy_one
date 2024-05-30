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
