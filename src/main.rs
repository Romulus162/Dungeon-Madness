use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
mod collisions;
mod debug;
mod input;
mod player;
mod systems;

// use input::PlayerInput;
use debug::DebugPlugin;

//remember movement and stuff

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
        .add_plugins((LdtkPlugin, RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0)))
        // .add_plugins(PlayerInput)
        .add_plugins(DebugPlugin)
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_systems(Startup, systems::setup)
        .add_systems(Update, collisions::spawn_wall_collision)
        // .add_systems(Update, systems::movement)
        .add_systems(Update, systems::camera_fit_inside_current_level)
        .add_systems(Update, player::player_movement)
        .add_systems(Update, collisions::spawn_ground_sensor)
        .add_systems(Update, collisions::ground_detection)
        // .add_systems(Update, systems::update_on_ground)
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .run();
}
