use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    WINDOW_SIZE,
    AppState,
};

const GAMEOVER_FONT_SIZE: f32 = 40.0;
const FONT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BG_SIZE: Vec2 = Vec2::new(160.0, 160.0);
const TEXT_GAP: f32 = 40.0;
const RESTART_FONT_SIZE: f32 = 30.0;

#[derive(Component)]
pub struct Gameover;

pub fn gameover_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Gameover
    commands.spawn((
        TextBundle::from_section(
            "Game Clear !!!",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: GAMEOVER_FONT_SIZE,
                color: FONT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - GAMEOVER_FONT_SIZE / 2.0 - TEXT_GAP),
            justify_self: JustifySelf::Center,
            ..default()
        }),
        Gameover,
    ));
    // Restart [R]
    commands.spawn((
        TextBundle::from_section(
            "Restart [R]",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: RESTART_FONT_SIZE,
                color: FONT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - RESTART_FONT_SIZE / 2.0 + TEXT_GAP),
            justify_self: JustifySelf::Center,
            ..default()
        }),
        Gameover,
    ));
    // Gameover background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BG_COLOR,
                custom_size: Some(BG_SIZE),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(
                    WINDOW_SIZE.x / 4.0,
                    WINDOW_SIZE.y / 4.0,
                    10.0
                ),
                ..default()
            },
            ..default()
        },
        Gameover,
    ));
}

pub fn gameover_update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gameover_query: Query<Entity, With<Gameover>>,
    level_selection: ResMut<LevelSelection>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
) {
    // R pressed
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        // Despawned gameover entities
        for gameover_entity in gameover_query.iter() {
            commands.entity(gameover_entity).despawn();
        }
        // Reset ldtk level
        let indices = match level_selection.into_inner() {
            LevelSelection::Indices(indices) => indices,
            _ => panic!("level selection should always be Indices in this game"),
        };
        indices.level = 0;
        // Moved app state to ingame
        app_state.set(AppState::InGame);
    }
}
