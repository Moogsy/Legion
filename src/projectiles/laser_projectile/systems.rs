use bevy::prelude::*;

use crate::projectiles::ShootEvent;

use super::{LaserProjectile, LaserProjectileBundle};

pub fn shoot(
    mut commands: Commands,
    mut reader: EventReader<ShootEvent<LaserProjectile>>,
    asset_server: Res<AssetServer>,
) {
    for event in reader.iter() {
        commands.spawn(LaserProjectileBundle::new(event, &asset_server));
    }
}