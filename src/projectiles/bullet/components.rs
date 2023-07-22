use bevy::prelude::*; 
use crate::{ai::{SmartUnitBundle, MoveState, Forward}, projectiles::ShootEvent};

use super::super::ProjectileType;

/// Marks this unit as a bullet shooter
#[derive(Component, Default)]
pub struct Bullet;
impl ProjectileType for Bullet {}

#[derive(Bundle, Default)]
pub struct BulletBundle {
    pub projectile_type: Bullet,
    pub smart_unit_bundle: SmartUnitBundle<MoveState<Forward>>,
    pub sprite_bundle: SpriteBundle,
}

impl BulletBundle {
    pub fn new(event: &ShootEvent<Bullet>, asset_server: &Res<AssetServer>) -> Self {
        let location = event.location;
        let orientation = event.orientation;

        Self { 
            projectile_type: Bullet, 
            sprite_bundle: SpriteBundle { 
                texture: asset_server.load("projectiles/bullet.png"),
                transform: Transform {
                    translation: location.extend(0.0),
                    rotation: orientation,
                    ..default()
                },
                ..default()
            },
            ..default()
        }
    }
}
