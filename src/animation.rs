use std::collections::HashMap;
use std::time::Duration;

use bevy::{prelude::*, asset};
use bevy_inspector_egui::egui::Key;
use bevy_rapier2d::prelude::*;

use crate::components::{GroundDetection, Player};

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
    CrouchTransition,
    Crouch,
    CrouchWalk,
    Attack,
    AttackSlide,
    Attack2,
    Attack2Slide,
    CrouchAttack,
    Combo,
    ComboSlide,
    SlideStart,
    Slide,
    SlideEnd,
    Roll,
    Dash,
    WallHang,
    WallSlide,
    WallClimb,
    WallClimbNoMovement,
    Death,
    DeathNoMovement,
}

#[derive(Debug, Clone, Component)]
struct AnimationMeta {
    len: usize,
    frame_time: f32,
    crouch_elapsed: f32,
}

impl AnimationMeta {
    fn new(len: usize, fps: usize) -> AnimationMeta {
        AnimationMeta {
            len: len,
            frame_time: 1.0 / (fps as f32),
            crouch_elapsed: 0.0,
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
            //10

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
            res.add(AnimationState::Fall, texture_atlas.add(fall_atlas), AnimationMeta::new(2, 1));

            let crouch_transition_atlas = TextureAtlas::from_grid(asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_CrouchTransition.png"),
                Vec2::new(120.0, 80.0),
                1,
                1,
                None,
                None
            );
            res.add(AnimationState::CrouchTransition, texture_atlas.add(crouch_transition_atlas), AnimationMeta::new(1, 12));

            let crouch = TextureAtlas::from_grid(asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Crouch.png"),
            Vec2::new(120.0, 80.0),
            1,
            1,
            None,
            None
            );
            res.add(AnimationState::Crouch, texture_atlas.add(crouch), AnimationMeta::new(1, 12));

            let crouch_walk_atlas = TextureAtlas::from_grid(
                asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_CrouchWalk.png"),
                Vec2::new(120.0, 80.0),
                8,
                1,
                None,
                None
            );
            res.add(AnimationState::CrouchWalk, texture_atlas.add(crouch_walk_atlas),AnimationMeta::new(7, 12));
            //8

            let attack = TextureAtlas::from_grid(
            asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_AttackNoMovement.png"),
            Vec2::new(120.0, 80.0),
            4,
            1,
            None,
            None
            );
            res.add(AnimationState::Attack, texture_atlas.add(attack),AnimationMeta::new(3,16));

            let attack_slide = TextureAtlas::from_grid(
                asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Attack.png"),
            Vec2::new(120.0, 80.0),
            4,
            1,
            None,
            None
            );
            res.add(AnimationState::AttackSlide, texture_atlas.add(attack_slide), AnimationMeta::new(3, 16));


            let attack2 = TextureAtlas::from_grid(
                asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Attack2NoMovement.png"),
                Vec2::new(120.0, 80.0),
                6,
                1,
                None,
                None
            );
            res.add(AnimationState::Attack2, texture_atlas.add(attack2), AnimationMeta::new(5, 16));

            let attack2_slide = TextureAtlas::from_grid(
                asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Attack2.png"),
                Vec2::new(120.0, 80.0),
                6,
                1,
                None,
                None
            );
            res.add(AnimationState::Attack2Slide, texture_atlas.add(attack2_slide), AnimationMeta::new(5, 16));

            let combo = TextureAtlas::from_grid(
                asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_AttackComboNoMovement.png"),
                Vec2::new(120.0, 80.0),
                10,
                1,
                None,
                None
            );
            res.add(AnimationState::Combo, texture_atlas.add(combo), AnimationMeta::new(9, 16));

            let combo_slide = TextureAtlas::from_grid(asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_AttackCombo2hit.png"),
            Vec2::new(120.0, 80.0),
            10,
            1,
            None,
            None);
            res.add(AnimationState::ComboSlide, texture_atlas.add(combo_slide), AnimationMeta::new(9, 16));

            let crouch_attack = TextureAtlas::from_grid(
                asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_CrouchAttack.png"),

            Vec2::new(120.0, 80.0),
            4,
            1,
            None,
            None
            );
            res.add(AnimationState::CrouchAttack, texture_atlas.add(crouch_attack),
             AnimationMeta::new(3, 16));

            let roll = TextureAtlas::from_grid(asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Roll.png"),
            Vec2::new(120.0, 80.0),
            12,
            1,
            None,
            None
            );
            res.add(AnimationState::Roll, texture_atlas.add(roll),
            AnimationMeta::new(11, 12));

            let dash = TextureAtlas::from_grid(asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Dash.png"),

            Vec2::new(120.0, 80.0),
            2,
            1,
            None,
            None
            );
            res.add(AnimationState::Dash, texture_atlas.add(dash),
            AnimationMeta::new(1,12));

            let slide = TextureAtlas::from_grid(asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_SlideFull.png"),
        Vec2::new(120.0, 80.0),
            4,
            1,
            None,
            None
            );
            res.add(AnimationState::Slide, texture_atlas.add(slide),
        AnimationMeta::new(3,6));

            let slide_start = TextureAtlas::from_grid(asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_SlideTransitionStart.png"),
            Vec2::new(120.0, 80.0),
            1,
            1,
            None,
            None);
            res.add(AnimationState::SlideStart, texture_atlas.add(slide_start), AnimationMeta::new(1, 12));

            let slide_end = TextureAtlas::from_grid(
                asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_SlideTransitionEnd.png"),
                Vec2::new(120.0, 80.0),
                1,
                1,
                None,
                None
            );
            res.add(AnimationState::SlideEnd, texture_atlas.add(slide_end), AnimationMeta::new(1, 12));

            let wall_hang = TextureAtlas::from_grid(
                asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_WallHang.png"),
                Vec2::new(120.0, 80.0),
                1,
                1,
                None,
                None
            );
            res.add(AnimationState::WallHang, texture_atlas.add(wall_hang),
        AnimationMeta::new(1, 12));

            let wall_climb = TextureAtlas::from_grid(asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_WallClimb.png"),
            Vec2::new(120.0, 80.0),
        7,
        1,
        None,
        None);
        res.add(AnimationState::WallClimb, texture_atlas.add(wall_climb),
            AnimationMeta::new(6, 12));

            let death = TextureAtlas::from_grid(asset_server.load("Knight/Colour1/Outline/120x80_PNGSheets/_Death.png"),
            Vec2::new(120.0, 80.0),
        10,
        1,
        None,
        None);
        res.add(AnimationState::Death, texture_atlas.add(death),
    AnimationMeta::new(9,12));
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
    mut animations: Query<(&mut TextureAtlasSprite, &mut AnimationMeta, &mut FrameTime)>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    for (mut sprite, mut animation, mut frame_time) in animations.iter_mut() {
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
        if input.pressed(KeyCode::S) {
            animation.crouch_elapsed += delt;
        } else {
            animation.crouch_elapsed = 0.0;
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

fn detect_wall() {}

fn change_player_animation(
    input: Res<Input<KeyCode>>,
    mut player: Query<
        (
            &Player,
            &mut Handle<TextureAtlas>,
            &mut AnimationMeta,
            &mut TextureAtlasSprite,
            &Velocity,
            &GroundDetection
        ),
        (With<Player>, With<AnimationMeta>),

    >,
    animations: Res<AnimationResource>,
) {


    if player.is_empty() {
        return;
    }
    let (_player, mut atlas, mut animation, mut sprite, velocity, ground_detection) = player.single_mut();



    if velocity.linvel.x < -0.1 {
        sprite.flip_x = true;
    } else if velocity.linvel.x > 0.1 {
        sprite.flip_x = false;
    }

    let mut set = AnimationState::Idle;

    let mut isSliding = false;

    if velocity.linvel.y > 0.01 {
        set = AnimationState::Jump
    } else if velocity.linvel.y < -0.01 {
        set =  AnimationState::Fall
    }
    else if input.pressed(KeyCode::S) && input.pressed(KeyCode::J){
        set = AnimationState::CrouchAttack;
    }
    else if input.pressed(KeyCode::S) && (input.pressed(KeyCode::D) || input.pressed(KeyCode::A)){
        set = AnimationState::CrouchWalk;
    }
    else if input.just_pressed(KeyCode::S){
        set = AnimationState::CrouchTransition;
    } else if input.pressed(KeyCode::S){
        set = AnimationState::Crouch;
    }
    else if input.pressed(KeyCode::L) && (!ground_detection.on_ground) {
        set = AnimationState::Dash;
    }

    /////////////////////////////

    //somewhere in this block, I need to add the slide start and slide end animations.

    // This overall code block is a mess and needs refactoring and further improvements

    else if input.pressed(KeyCode::L) && (input.pressed(KeyCode::A) || input.pressed(KeyCode::D) ){
        isSliding = true;
        if isSliding {
            set = AnimationState::SlideStart;
            isSliding = true;
            if isSliding {
                set = AnimationState::Slide;
            } else {
                isSliding = false;
                set = AnimationState::SlideEnd;
            }
        }
        // set = AnimationState::SlideStart;
        // set = AnimationState::Slide;
        // set = AnimationState::SlideEnd;
    }
//////////////////////////////////////////


// let mut isSliding = false;
// let mut startSliding = false;
// let mut endSlide = false;


    else if input.pressed(KeyCode::L){
        set = AnimationState::Roll;
    }
    else if velocity.linvel.x != 0.0 {
        set = AnimationState::Run
    }
    else if (input.pressed(KeyCode::D) || input.pressed(KeyCode::A)) && input.pressed(KeyCode::J) && input.pressed(KeyCode::K){
        set = AnimationState::ComboSlide;
    }
    else if input.pressed(KeyCode::J) && input.pressed(KeyCode::K){
        set = AnimationState::Combo;
    }
    else if (input.pressed(KeyCode::A) || input.pressed(KeyCode::D)) && input.pressed(KeyCode::J){
        set = AnimationState::AttackSlide;
    }
    else if input.pressed(KeyCode::J){
        set = AnimationState::Attack
    }
    else if (input.pressed(KeyCode::A) || input.pressed(KeyCode::D)) && input.pressed(KeyCode::K){
        set = AnimationState::Attack2Slide;
    }
    else if input.pressed(KeyCode::K){
        set = AnimationState::Attack2;
    }
    else if input.pressed(KeyCode::I) {
        set = AnimationState::Death
    }


    println!("Current Animation State: {:?}", set);

    let Some((new_atlas, new_animation)) = animations.get(set) else {
        error!("No Animation Jump Loaded");
        return;
    };
    *atlas = new_atlas;
    sprite.index %= new_animation.len;
    *animation = new_animation;
}


//
