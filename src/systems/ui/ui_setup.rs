use bevy::{ecs::{system::Commands, component::Component}, ui::{node_bundles::{NodeBundle, TextBundle}, Style, Val, JustifyContent, UiRect, AlignItems, Direction, JustifyItems}, prelude::default, hierarchy::BuildChildren, render::color::Color, text::TextStyle};

use super::{theme::{BOARDER_COLOR, BACKGROUND_COLOR, TEXT_COLOR}, ui_continuous::HexPosText};

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
        root.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(20.),
                height: Val::Percent(100.),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BOARDER_COLOR.into(),        
            ..default()
        }).with_children(|info_pane_boarder| {
            info_pane_boarder.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BACKGROUND_COLOR.into(),
                ..default()
            }).with_children(|info_pane_content| {
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

#[derive(Component)]
pub struct UICompass;