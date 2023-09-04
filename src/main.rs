use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod input;
mod debug;
mod components;
mod systems;

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
        .add_plugins(LdtkPlugin)
        .add_plugins(PlayerInput)
        .add_plugins(DebugPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::Uid(0))
        .run();
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);

    let ldtk_handle = asset_server.load("Typical_2D_platformer_example.ldtk");
    commands.spawn(LdtkWorldBundle { ldtk_handle, ..Default::default() });
}
