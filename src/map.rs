use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;

use crate::ascii::{spawn_ascii_sprite, AsciiSheet};

pub const TILE_SIZE: f32 = 32.0;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_map);
    }
}

#[derive(Component)]
pub struct Map;

fn spawn_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let file = File::open("assets/map.txt").expect("No map file found!");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, tile) in line.chars().enumerate() {
                print!("{tile}");
                let color = match tile {
                    '#' => Color::rgb(0.7, 0.7, 0.7),
                    '@' => Color::rgb(0.5, 0.5, 0.2),
                    '~' => Color::rgb(0.2, 0.9, 0.2),
                    _ => Color::rgb(0.9, 0.9, 0.9),
                };
                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    tile as usize,
                    color,
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                    Vec3::splat(1.0),
                );
                tiles.push(tile);
            }
        }
    }

    commands
        .spawn_empty()
        .insert(Map)
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Visibility::Visible)
        .insert(ComputedVisibility::default())
        .push_children(&tiles);
}
