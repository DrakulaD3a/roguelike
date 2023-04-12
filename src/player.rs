use std::collections::VecDeque;

use bevy::prelude::*;

use crate::timer::TickState;

pub const MAX_INPUT_LENGHT: usize = 4;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_startup_system(inicialize_player_input)
            .add_system(receive_input.in_set(OnUpdate(TickState::Input)))
            .add_system(execute_input.in_set(OnUpdate(TickState::Execution)));
    }
}

#[derive(Component)]
pub struct Player;

pub const PLAYER_SIZE: f32 = 50.0;

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

#[derive(Resource, Debug)]
pub struct PlayerInput(VecDeque<KeyCode>);

fn inicialize_player_input(mut commands: Commands) {
    commands.insert_resource(PlayerInput(VecDeque::new()));
}

fn receive_input(input: Res<Input<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    if player_input.0.len() > MAX_INPUT_LENGHT {
        player_input.0.pop_front();
    }

    player_input
        .0
        .push_back(match input.get_just_pressed().next() {
            Some(&key) => key,
            None => return,
        });
}

fn execute_input(mut tick_state: ResMut<NextState<TickState>>) {
    println!("Execute input");
    tick_state.set(TickState::Input);
}
