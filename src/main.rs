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
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // .add_plugins(
        //     DefaultPlugins.set(ImagePlugin::default_nearest())
        //         .set(WindowPlugin {
        //             primary_window: Some(Window {
        //                 title: "Dungeon-Madness".into(),
        //                 resolution: (640.0, 480.0).into(),
        //                 resizable: false,
        //                 ..default()
        //             }),
        //             ..default()
        //         })
        //         .build()
        // )
        .add_plugins(LdtkPlugin)
        // .add_plugins(PlayerInput)
        // .add_plugins(DebugPlugin)
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, systems::camera_fit_inside_current_level)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("Typical_2D_platformer_example.ldtk"),
        ..Default::default()
    });

    //NO ATLAS
    // commands.spawn(LdtkWorldBundle {
    //     ldtk_handle: asset_server.load("DevMap.ldtk"),
    //     ..Default::default()
    // });
}
