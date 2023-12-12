use bevy::ecs::component::Component;

#[derive(Component)]
pub struct UICompass;

#[derive(Component)]
pub struct HexPosText;

#[derive(Component)]
pub struct RightInfoPane;

#[derive(Component)]
pub enum ButtonOnClick {
    InfoPaneClose
}