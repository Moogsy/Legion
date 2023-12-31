use bevy::{prelude::*, window::PrimaryWindow};

use super::GunnerBundle;

pub fn test_spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.single();
    let x = window.width() / 4.0;
    let y = window.height() / 2.0;

    commands.spawn(GunnerBundle::new(Transform::from_xyz(x, y, 0.0), &asset_server));
}
