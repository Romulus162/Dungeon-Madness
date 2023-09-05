use bevy::prelude::*;
use bevy_ecs_ldtk::{ prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted };
use std::collections::HashSet;
use bevy_rapier2d::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub density: ColliderMassProperties,
}

impl From<&EntityInstance> for ColliderBundle {
    fn from(entity_instance: &EntityInstance) -> ColliderBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" =>
                ColliderBundle {
                    collider: Collider::cuboid(6.0, 14.0),
                    rigid_body: RigidBody::Dynamic,
                    friction: Friction {
                        coefficient: 0.0,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    rotation_constraints,
                    ..Default::default()
                },
            _ => ColliderBundle::default(),
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
}