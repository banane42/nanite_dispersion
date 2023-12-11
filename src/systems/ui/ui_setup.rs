use bevy::{ecs::{system::Commands, component::Component}, ui::{node_bundles::NodeBundle, Style, Val, JustifyContent, UiRect, AlignItems}, prelude::default, hierarchy::BuildChildren, render::color::Color};

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
            background_color: Color::rgb(0.65, 0.65, 0.65).into(),
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
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
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
    });
}

#[derive(Component)]
pub struct UICompass;