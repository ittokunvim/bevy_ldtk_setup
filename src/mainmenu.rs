use bevy::prelude::*;

use crate::{
    GAMETITLE,
    AppState,
};

const GAMETITLE_FONT_SIZE: f32 = 40.0;
const CLICKSTART_FONT_SIZE: f32 = 30.0;
const FONT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Component)]
pub struct Mainmenu;

pub fn mainmenu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    // Game title
    commands.spawn((
        TextBundle::from_section(
            GAMETITLE,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: GAMETITLE_FONT_SIZE,
                color: FONT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            ..default()
        }),
        Mainmenu,
    ));
    // Background image
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/thumbnail.png"),
            ..default()
        },
        Mainmenu,
    ));
    // Click start
    commands.spawn((
        TextBundle::from_section(
            "click start ...",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: CLICKSTART_FONT_SIZE,
                color: FONT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Px(16.0),
            bottom: Val::Px(16.0),
            ..default()
        }),
        Mainmenu,
    ));
}

pub fn mainmenu_update(
    mouse_event: Res<ButtonInput<MouseButton>>,
    mainmenu_query: Query<Entity, With<Mainmenu>>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if mouse_event.just_pressed(MouseButton::Left) {
        // Despawned mainmenu
        for mainmenu_entity in mainmenu_query.iter() {
            commands.entity(mainmenu_entity).despawn();
        }
        // Changed app state
        app_state.set(AppState::InGame);
    }
}
