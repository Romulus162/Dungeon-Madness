use crate::components::*;
use bevy::{ prelude::*, render::render_resource::Texture };
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::egui::Key;
use bevy::{ prelude::*, utils::HashMap };
// window::PrimaryWindow };
use bevy_rapier2d::prelude::{ Collider, RigidBody };
use bevy_rapier2d::dynamics::GravityScale;
use bevy_rapier2d::control::CharacterAutostep;

use bevy_rapier2d::prelude::*;

pub fn player_movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &GroundDetection, &mut TextureAtlasSprite, &mut GravityScale), With<Player>>
) {
    for (mut velocity, ground_detection,sprite, mut gravity) in &mut query {
        let right = if input.pressed(KeyCode::D) { 1.0 } else { 0.0 };

        let left = if input.pressed(KeyCode::A) { 1.0 } else { 0.0 };

        let mut speed_multiplier = 1.0;


        if input.pressed(KeyCode::J) || input.pressed(KeyCode::K){
            speed_multiplier = 0.0;
        }
        else if input.pressed(KeyCode::S){
            speed_multiplier = 0.3;
        }

        ////////////////////////////////

        if input.pressed(KeyCode::L) && (!ground_detection.on_ground) {
            *gravity = GravityScale(0.0);
            let dodge_direction = if velocity.linvel.x >= 0.0 { 1.0 } else { -1.0 };
            velocity.linvel.x += 200.0 * dodge_direction;
            velocity.linvel.y = 0.0;
        } else {
            *gravity = GravityScale(1.0);
        }
//////////////////////////////////////////////
        velocity.linvel.x = (right - left) * 200.0 * speed_multiplier;

        if input.pressed(KeyCode::L){
            let dodge_direction = if velocity.linvel.x >= 0.0 { 1.0 } else { -1.0 };
            velocity.linvel.x += 200.0 * dodge_direction;
        }


        if input.just_pressed(KeyCode::W) && ground_detection.on_ground {
            velocity.linvel.y = 500.0;
        }



    }
}
