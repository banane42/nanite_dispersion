use bevy::{ecs::{system::Commands, component::Component}, ui::{node_bundles::{NodeBundle, TextBundle, ButtonBundle}, Style, Val, JustifyContent, UiRect, AlignItems, Direction, JustifyItems, FlexDirection, AlignContent, PositionType}, prelude::default, hierarchy::BuildChildren, render::{color::Color, view::Visibility}, text::{TextStyle, TextAlignment}};

use crate::components::ui::{UICompass, HexPosText, RightInfoPane, ButtonOnClick};

use super::theme::{BOARDER_COLOR, BACKGROUND_COLOR, TEXT_COLOR};

pub fn ui_setup(
    mut commands: Commands
) {
    //ROOT Node
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    }).with_children(|root| {
        // compass box border
        root.spawn(NodeBundle {
            style: Style {
                width: Val::Px(50.),
                height: Val::Px(50.),
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

        //Right Info
        root.spawn((NodeBundle {
            style: Style {
                width: Val::Percent(20.),
                height: Val::Percent(100.),
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
                info_pane_content.spawn((TextBundle::from_section(
                    "Text", 
                    TextStyle {
                        color: TEXT_COLOR.into(),
                        ..default()
                    }
                ), HexPosText));
            });
        });
    });
}