use bevy::{prelude::*, window::PrimaryWindow};
use legion::{ProjectilesPlugin, SpaceshipsPlugin, SpatialPlugin}; 

fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
)
{
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            window.width() / 2.0,
            window.height() / 2.0,
            1000.0,
        ),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpatialPlugin)
        .add_plugins(SpaceshipsPlugin)
        .add_plugins(ProjectilesPlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}
