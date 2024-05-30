use crate::main_menu::{
    components::{MainMenu, PlayButton, QuitButton},
    styles::*,
};
use bevy::prelude::*;
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, mm_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = mm_query.get_single() {
        // recursive = deapwns current entity and all its children
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

// Not a system. just a rust fn
pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    column_gap: Val::Px(8.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::RED),
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // -- Title --
            //
            // -- Play Btn --
            parent
                .spawn((
                    ButtonBundle {
                        style: BTN_STYLE,
                        background_color: BackgroundColor(NORMAL_BTN_COLOR),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                TextStyle {
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });
            //  -- Quit Btn --
            parent
                .spawn((
                    ButtonBundle {
                        style: BTN_STYLE,
                        background_color: BackgroundColor(NORMAL_BTN_COLOR),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                TextStyle {
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    main_menu_entity
}
