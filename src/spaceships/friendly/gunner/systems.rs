use bevy::{prelude::*, window::PrimaryWindow};

use crate::{projectiles::{ShootEvent, Bullet}, ai::ShootState};

use super::{GunnerBundle, Gunner}; 

pub fn test_spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.single();
    let x = window.width() / 2.0;
    let y = window.height() / 2.0;

    commands.spawn(
        GunnerBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("friendlies/gunner.png"),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            ..default()
        }
    );
}

pub fn shoot(
    mut writer: EventWriter<ShootEvent<Bullet>>,
    query: Query<(&Transform, &Handle<Image>), (With<Gunner>, With<ShootState>)>,
    assets: Res<Assets<Image>>,
) {
    for (transform, image_handle) in query.iter() {
        let image = assets.get(image_handle).unwrap(); 
        let size = image.texture_descriptor.size;

        writer.send(
            ShootEvent::<Bullet>::in_front_of(transform, &size)
        )
    }
}