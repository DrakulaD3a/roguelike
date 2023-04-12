use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};

mod player;
mod timer;

fn main() {
    App::new()
        // Plugin set for spawning window
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Strategy".to_string(),
                        present_mode: bevy::window::PresentMode::Fifo,
                        resolution: WindowResolution::new(1280.0, 720.0)
                            .with_scale_factor_override(1.0),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(player::PlayerPlugin)
        .add_plugin(timer::TimerPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..Default::default()
        })
        .insert(Name::new("Camera"));
}
