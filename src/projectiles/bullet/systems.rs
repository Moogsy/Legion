use bevy::prelude::*;

use super::{super::ShootEvent, Bullet, components::BulletBundle};



pub fn shoot(
    mut commands: Commands,
    mut reader: EventReader<ShootEvent<Bullet>>,
    asset_server: Res<AssetServer>
) {
    for event in reader.iter() {
        commands.spawn(BulletBundle::new(event, &asset_server));
    }
}