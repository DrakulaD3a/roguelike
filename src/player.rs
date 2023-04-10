use bevy::{prelude::*, window::PrimaryWindow};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(gravity)
            .add_system(confine_player_movement);
    }
}

#[derive(Component)]
pub struct Player;

const PLAYER_SIZE: f32 = 50.0;

fn spawn_player(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .insert(Player);
}

const PLAYER_SPEED: f32 = 500.0;
const G: f32 = 100.0;

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 10.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -10.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-10.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(10.0, 0.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

fn gravity(mut player_query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        transform.translation.y -= G * time.delta_seconds();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        if transform.translation.x < PLAYER_SIZE / 2.0 {
            transform.translation.x = PLAYER_SIZE / 2.0;
        } else if transform.translation.x > window.width() as f32 - PLAYER_SIZE / 2.0 {
            transform.translation.x = window.width() as f32 - PLAYER_SIZE / 2.0;
        }

        if transform.translation.y < PLAYER_SIZE / 2.0 {
            transform.translation.y = PLAYER_SIZE / 2.0;
        } else if transform.translation.y > window.height() as f32 - PLAYER_SIZE / 2.0 {
            transform.translation.y = window.height() as f32 - PLAYER_SIZE / 2.0;
        }
    }
}
