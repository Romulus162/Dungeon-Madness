use bevy::prelude::*;

mod input;
mod debug;

use input::PlayerInput;
use debug::DebugPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Dungeon-Madness".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build()
        )
        .add_systems(Startup, setup)
        .add_plugins(PlayerInput)
        .add_plugins(DebugPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
