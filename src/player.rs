use std::collections::VecDeque;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::timer::TickState;

pub const MAX_INPUT_LENGHT: usize = 4;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_startup_system(inicialize_player_input)
            .add_system(player_movement)
            .add_system(confine_player_movement)
            .add_system(receive_input.in_set(OnUpdate(TickState::Input)))
            .add_system(execute_input.in_set(OnUpdate(TickState::Execution)));
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

#[derive(Resource, Debug)]
pub struct PlayerInput(VecDeque<KeyCode>);

fn inicialize_player_input(mut commands: Commands) {
    commands.insert_resource(PlayerInput(VecDeque::new()));
}

fn receive_input(input: Res<Input<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    if player_input.0.len() > MAX_INPUT_LENGHT {
        player_input.0.pop_front();
    }

    player_input.0.push_back(match input.get_just_pressed().next() {
        Some(&key) => key,
        None => return,
    });
}

fn execute_input(mut tick_state: ResMut<NextState<TickState>>) {
    println!("Execute input");
    tick_state.set(TickState::Input);
}
