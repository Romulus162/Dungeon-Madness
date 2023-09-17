use crate::components::*;
use bevy::{ prelude::*, render::render_resource::Texture };
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::egui::Key;
use bevy::{ prelude::*, utils::HashMap };
// window::PrimaryWindow };
use bevy_rapier2d::prelude::{ Collider, RigidBody };

use bevy_rapier2d::prelude::*;

pub fn player_movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &GroundDetection), With<Player>>
) {
    for (mut velocity, ground_detection) in &mut query {
        let right = if input.pressed(KeyCode::D) { 1.0 } else { 0.0 };

        let left = if input.pressed(KeyCode::A) { 1.0 } else { 0.0 };

        velocity.linvel.x = (right - left) * 200.0;

        if input.just_pressed(KeyCode::W) && ground_detection.on_ground {
            velocity.linvel.y = 500.0;
        }
    }
}
