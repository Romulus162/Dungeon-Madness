use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::Player;

#[derive(Debug)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AnimationResource>()
            .add_systems(Update, animate_sprite)
            .add_systems(Update, append_animation_for_player)
            .add_systems(Update, change_player_animation);
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum AnimationState {
    Idle,
    Jump,
    Run,
    Fall,
}

#[derive(Debug, Clone, Component)]
struct AnimationMeta {
    len: usize,
    frame_time: f32,
}

impl AnimationMeta {
    fn new(len: usize, fps: usize) -> AnimationMeta {
        AnimationMeta {
            len: len,
            frame_time: 1.0 / (fps as f32),
        }
    }
}

#[derive(Debug, Resource)]
struct AnimationResource {
    map: HashMap<AnimationState, (Handle<TextureAtlas>, AnimationMeta)>,
}

impl AnimationResource {
    fn add(&mut self, state: AnimationState, handle: Handle<TextureAtlas>, meta: AnimationMeta) {
        self.map.insert(state, (handle, meta));
    }
    fn get(&self, state: AnimationState) -> Option<(Handle<TextureAtlas>, AnimationMeta)> {
        self.map.get(&state).cloned()
    }
}

impl FromWorld for AnimationResource {
    fn from_world(world: &mut World) -> Self {
        let mut res = AnimationResource {
            map: HashMap::new(),
        };
        world.resource_scope(|world, mut texture_atlas: Mut<Assets<TextureAtlas>>| {
            let asset_server = world.resource::<AssetServer>();

            let idle_atlas = TextureAtlas::from_grid(
                asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Idle.png"),
                Vec2::new(120.0, 80.0),
                10,
                1,
                None,
                None
            );
            res.add(AnimationState::Idle, texture_atlas.add(idle_atlas), AnimationMeta::new(9, 12));

            let run_atlas = TextureAtlas::from_grid(
                asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Run.png"),
                Vec2::new(120.0, 80.0),
                10,
                1,
                None,
                None
            );
            res.add(AnimationState::Run, texture_atlas.add(run_atlas), AnimationMeta::new(9, 12));

            let jump_atlas = TextureAtlas::from_grid(
                asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Jump.png"),
                Vec2::new(120.0, 80.0),
                3,
                1,
                None,
                None
            );
            res.add(AnimationState::Jump, texture_atlas.add(jump_atlas), AnimationMeta::new(2, 12));

            let fall_atlas = TextureAtlas::from_grid(
                asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Fall.png"),
                Vec2::new(120.0, 80.0),
                3,
                1,
                None,
                None
            );
            res.add(AnimationState::Fall, texture_atlas.add(fall_atlas), AnimationMeta::new(2, 12))
        });
        res
    }
}

#[derive(Component)]
struct FrameTime(pub f32);

#[derive(Bundle)]
pub struct PhoxAnimationBundle {
    animation: AnimationMeta,
    frame_time: FrameTime,
}

impl PhoxAnimationBundle {
    fn new(animation: AnimationMeta) -> PhoxAnimationBundle {
        PhoxAnimationBundle {
            animation,
            frame_time: FrameTime(0.0),
        }
    }
}

fn animate_sprite(
    mut animations: Query<(&mut TextureAtlasSprite, &AnimationMeta, &mut FrameTime)>,
    time: Res<Time>
) {
    for (mut sprite, animation, mut frame_time) in animations.iter_mut() {
        let delt = time.delta_seconds();
        frame_time.0 += delt;
        if frame_time.0 > animation.frame_time {
            let frames = (frame_time.0 / animation.frame_time) as usize;
            sprite.index += frames;
            if sprite.index >= animation.len {
                sprite.index %= animation.len;
            }
            frame_time.0 -= animation.frame_time;
        }
    }
}

fn append_animation_for_player(
    mut commands: Commands,
    mut query: Query<Entity, (With<Player>, Without<AnimationMeta>)>,
    animations: Res<AnimationResource>
) {
    if query.is_empty() {
        return;
    }
    let entity = query.single_mut();

    let Some((_texture_atlas, animation)) = animations.get(AnimationState::Idle) else {
        error!("Failed to find animation: Idle");
        return;
    };
    commands.entity(entity).insert(PhoxAnimationBundle::new(animation));
}

fn change_player_animation(
    mut player: Query<
        (
            &Player,
            &mut Handle<TextureAtlas>,
            &mut AnimationMeta,
            &mut TextureAtlasSprite,
            &Velocity,
        ),
        (With<Player>, With<AnimationMeta>)
    >,
    animations: Res<AnimationResource>
) {
    if player.is_empty() {
        return;
    }
    let (_player, mut atlas, mut animation, mut sprite, velocity) = player.single_mut();
    if velocity.linvel.x < -0.1 {
        sprite.flip_x = true;
    } else if velocity.linvel.x > 0.1 {
        sprite.flip_x = false;
    }

    let set = if velocity.linvel.y > 0.01 {
        AnimationState::Jump
    } else if velocity.linvel.y < -0.01 {
        AnimationState::Fall
    } else if velocity.linvel.x != 0.0 {
        AnimationState::Run
    } else {
        AnimationState::Idle
    };

    let Some((new_atlas, new_animation)) = animations.get(set) else {
        error!("No Animation Jump Loaded");
        return;
    };
    *atlas = new_atlas;
    sprite.index %= new_animation.len;
    *animation = new_animation;
}
