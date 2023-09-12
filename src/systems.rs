use crate::components::*;
use bevy::{ prelude::*, render::render_resource::Texture };
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::egui::Key;

use std::{ collections::{ HashMap, HashSet }, f32::consts::E };

use bevy_rapier2d::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("Dungeon.ldtk"),
        ..Default::default()
    });

    //NO ATLAS
    // commands.spawn(LdtkWorldBundle {
    //     ldtk_handle: asset_server.load("DevMap.ldtk"),
    //     ..Default::default()
    // });
}

const ASPECT_RATIO: f32 = 16.0 / 9.0;

pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (&mut bevy::render::camera::OrthographicProjection, &mut Transform),
        Without<Player>
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>)
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>
) {
    if let Ok(Transform { translation: player_translation, .. }) = player_query.get_single() {
        let player_translation = *player_translation;

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in &level_query {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, &level) {
                    let level_ratio = (level.px_wid as f32) / (ldtk_level.level.px_hei as f32);
                    orthographic_projection.viewport_origin = Vec2::ZERO;
                    if level_ratio > ASPECT_RATIO {
                        let height = ((level.px_hei as f32) / 9.0).round() * 9.0;
                        let width = height * ASPECT_RATIO;
                        orthographic_projection.scaling_mode =
                            bevy::render::camera::ScalingMode::Fixed {
                                width,
                                height,
                            };
                        camera_transform.translation.x = (
                            player_translation.x -
                            level_transform.translation.x -
                            width / 2.0
                        ).clamp(0.0, (level.px_wid as f32) - width);
                        camera_transform.translation.y = 0.0;
                    } else {
                        let width = ((level.px_wid as f32) / 16.0).round() * 16.0;
                        let height = width / ASPECT_RATIO;
                        orthographic_projection.scaling_mode =
                            bevy::render::camera::ScalingMode::Fixed {
                                width,
                                height,
                            };
                        camera_transform.translation.y = (
                            player_translation.y -
                            level_transform.translation.y -
                            height / 2.0
                        ).clamp(0.0, (level.px_hei as f32) - height);
                        camera_transform.translation.x = 0.0;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
            }
        }
    }
}
