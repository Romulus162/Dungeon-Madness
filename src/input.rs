use bevy::prelude::*;

pub struct PlayerInput;
impl Plugin for PlayerInput {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, player_movement);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("Knight/Colour1/NoOutline/120x80_PNGSheets/_Crouch.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(120.0, 80.0)),
            ..default()
        },
        texture,
        ..default()
    });
}

// fn player_movement(
//     mut characters: Query<(&mut Transform, &Sprite)>,
//     input: Res<Input<KeyCode>>,
//     time: Res<Time>
// ) {
//     for (mut transform, _) in &mut characters {
//         if input.pressed(KeyCode::W) {
//             transform.translation.y += 100.0 * time.delta_seconds();
//         }
//         if input.pressed(KeyCode::S) {
//             transform.translation.y -= 100.0 * time.delta_seconds();
//         }
//         if input.pressed(KeyCode::D) {
//             transform.translation.x += 100.0 * time.delta_seconds();
//         }
//         if input.pressed(KeyCode::A) {
//             transform.translation.x -= 100.0 * time.delta_seconds();
//         }
//     }
// }
fn player_movement(
    mut characters: Query<(&mut Transform, &Sprite)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    for (mut transform, _) in &mut characters {
        if input.pressed(KeyCode::W) {
            transform.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += 100.0 * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= 100.0 * time.delta_seconds();
        }
    }
}
