use crate::components::*;
use bevy::{ prelude::*, render::render_resource::Texture };
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::egui::Key;
use bevy::{ prelude::*, utils::HashMap };
// window::PrimaryWindow };
use bevy_rapier2d::prelude::{ Collider, RigidBody };

use bevy_rapier2d::prelude::*;

pub fn player_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::W) {
            transform.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
    }
}
