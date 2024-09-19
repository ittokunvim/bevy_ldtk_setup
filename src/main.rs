use bevy::prelude::*;

const GAMETITLE: &str = "Bevy LDtk Setup";
const WINDOW_SIZE: Vec2 = Vec2::new(800.0, 800.0);
const BG_COLOR: Color = Color::srgb(0.255, 0.251, 0.333);

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
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
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
