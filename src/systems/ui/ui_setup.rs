use bevy::{ecs::system::Commands, ui::{node_bundles::{NodeBundle, TextBundle, ButtonBundle}, Style, Val, JustifyContent, UiRect, AlignItems, FlexDirection, AlignContent, PositionType, FlexWrap, Display}, prelude::default, hierarchy::BuildChildren, render::{color::Color, view::Visibility}, text::{TextStyle, TextAlignment, Text}};

use crate::components::ui::{UICompass, HexPosText, RightInfoPane, ButtonOnClick, HexTerrainText, HexNaniteText};

use super::theme::{BOARDER_COLOR, BACKGROUND_COLOR, TEXT_COLOR};

pub fn ui_setup(
    mut commands: Commands
) {
    //ROOT Node
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexEnd,
            ..default()
        },
        ..default()
    }).with_children(|root| {
        // compass box border
        root.spawn(NodeBundle {
            style: Style {
                width: Val::Px(50.),
                height: Val::Px(50.),
                left: Val::Px(8.),
                top: Val::Px(8.),
                position_type: PositionType::Absolute,
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            background_color: BOARDER_COLOR.into(),
            ..default()
        }).with_children(|parent| {
            // compass box content
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BACKGROUND_COLOR.into(),
                ..default()
            }).with_children(|compass_box| {
                //Compass
                compass_box.spawn((NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        height: Val::Percent(10.0),
                        justify_content: JustifyContent::End,
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                },
                UICompass
            )).with_children(|compass| {
                    //Red tip
                    compass.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(30.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: Color::RED.into(),
                        ..default()
                    });
                });
            });
        });

        //Spacer
        root.spawn(NodeBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Auto,
                ..default()
            },
            ..default()
        });

        // Right Content
        root.spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_wrap: FlexWrap::Wrap,
                align_content: AlignContent::Stretch,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }).with_children(|right_content| {
            //Right Info
            right_content.spawn((NodeBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    flex_grow: 1.0,
                    // margin: UiRect::vertical(Val::Px(24.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                visibility: Visibility::Hidden,
                background_color: BOARDER_COLOR.into(),        
                ..default()
            }, RightInfoPane)
            ).with_children(|info_pane_content| {
                info_pane_content.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        justify_content: JustifyContent::Start,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                }).with_children(|info_pane_content| {
                    //Close Button
                    info_pane_content.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(25.),
                            height: Val::Px(25.),
                            left: Val::Px(4.),
                            top: Val::Px(4.),
                            position_type: PositionType::Absolute,
                            border: UiRect::all(Val::Px(1.0)),
                            justify_content: JustifyContent::Center,
                            align_content: AlignContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        border_color: BOARDER_COLOR.into(),
                        ..default()
                    }, ButtonOnClick::InfoPaneClose))
                    .with_children(|close_button| {
                        close_button.spawn((TextBundle::from_section(
                            "X", 
                            TextStyle {
                                color: TEXT_COLOR.into(),
                                ..default()
                            }
                        )).with_text_alignment(TextAlignment::Center));
                    });
                    // Grid Pos Text
                    info_pane_content.spawn((TextBundle {
                        text: Text::from_section("", TextStyle {
                            color: TEXT_COLOR.into(),
                            ..default()
                        }).with_alignment(TextAlignment::Center),
                        style: Style {
                            margin: UiRect::axes(Val::Px(8.), Val::Px(12.)),
                            ..default()
                        },
                        ..default()
                    }, HexPosText));

                    //Hex Terrain Info
                    info_pane_content.spawn((TextBundle {
                        text: Text::from_section("", TextStyle {
                            color: TEXT_COLOR.into(),
                            ..default()
                        }).with_alignment(TextAlignment::Center),
                        style: Style {
                            margin: UiRect::axes(Val::Px(8.), Val::Px(12.)),
                            ..default()
                        },
                        ..default()
                    }, HexTerrainText));

                    //Hex Terrain Info
                    info_pane_content.spawn((TextBundle {
                        text: Text::from_section("", TextStyle {
                            color: TEXT_COLOR.into(),
                            ..default()
                        }).with_alignment(TextAlignment::Center),
                        style: Style {
                            margin: UiRect::axes(Val::Px(8.), Val::Px(12.)),
                            ..default()
                        },
                        ..default()
                    }, HexNaniteText));
                });
            });

            // Map Buttons
            right_content.spawn(NodeBundle {
                style: Style {
                    align_content: AlignContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexEnd,
                    align_self: bevy::ui::AlignSelf::End,
                    flex_grow: 0.0,
                    ..default()
                },
                background_color: Color::RED.into(),
                ..default()
            }).with_children(|map_button_container| {
                // Terrain Button
                map_button_container.spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(25.),
                        height: Val::Px(25.),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    border_color: BOARDER_COLOR.into(),
                    ..default()
                }, ButtonOnClick::MapButtonTerrain))
                .with_children(|close_button| {
                    close_button.spawn((TextBundle::from_section(
                        "T", 
                        TextStyle {
                            color: TEXT_COLOR.into(),
                            ..default()
                        }
                    )).with_text_alignment(TextAlignment::Center));
                });

                // Nanite Button
                map_button_container.spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(25.),
                        height: Val::Px(25.),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    border_color: BOARDER_COLOR.into(),
                    ..default()
                }, ButtonOnClick::MapButtonNanite))
                .with_children(|close_button| {
                    close_button.spawn((TextBundle::from_section(
                        "N", 
                        TextStyle {
                            color: TEXT_COLOR.into(),
                            ..default()
                        }
                    )).with_text_alignment(TextAlignment::Center));
                });
            });
        });
    });
}