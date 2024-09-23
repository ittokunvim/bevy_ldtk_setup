use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod mainmenu;
mod ingame;
mod gameover;

use crate::mainmenu::{
    mainmenu_setup,
    mainmenu_update,
};

use crate::ingame::{
    PlayerBundle,
    WallBundle,
    GoalBundle,
    LevelWalls,
    ingame_setup,
    move_player_from_input,
    translate_grid_coords_entities,
    cache_wall_locations,
    check_goal,
};

use crate::gameover::{
    gameover_setup,
    gameover_update,
};

const GAMETITLE: &str = "Bevy LDtk Setup";
const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 800.0);
const BG_COLOR: Color = Color::srgb(0.255, 0.251, 0.333);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    title: GAMETITLE.to_string(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .init_state::<AppState>()
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        // ldtk setup
        .add_plugins(LdtkPlugin)
        .init_resource::<LevelWalls>()
        .insert_resource(LevelSelection::index(0))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<GoalBundle>("Goal")
        .register_ldtk_int_cell::<WallBundle>(1)
        // mainmenu
        .add_systems(OnEnter(AppState::MainMenu), mainmenu_setup)
        .add_systems(Update, mainmenu_update.run_if(in_state(AppState::MainMenu)))
        // ingame
        .add_systems(OnEnter(AppState::InGame), ingame_setup)
        .add_systems(Update, (
            move_player_from_input,
            translate_grid_coords_entities,
            cache_wall_locations,
            check_goal,
            // update_ingame,
        ).run_if(in_state(AppState::InGame)))
        // gameover
        .add_systems(OnEnter(AppState::GameOver), gameover_setup)
        .add_systems(Update, gameover_update.run_if(in_state(AppState::GameOver)))
        .run();
}
