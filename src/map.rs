use bevy::prelude::*;

use crate::player::PLAYER_SIZE;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_map);
    }
}

#[derive(Component, Reflect)]
pub struct Tile;

fn spawn_map(mut commands: Commands) {
    for i in 0..100 {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                    ..Default::default()
                },
                transform: Transform::from_xyz((i / 10) as f32 * PLAYER_SIZE, 0.0, (i % 10) as f32 * PLAYER_SIZE),
                ..Default::default()
            })
            .insert(Name::new("Tile"))
            .insert(Tile);
    }
}
