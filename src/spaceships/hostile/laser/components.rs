use bevy::prelude::*;

use crate::{spaceships::{Legion, ShipIdentificationBundle, ShipClass, ShipCapabilities}, ai::{SmartUnitBundle, ShootAction, CanShoot, CanTranslate, CanRotate}, projectiles::LaserProjectile}; 

/// Enemy that shoots laser blasts
#[derive(Component, Default)]
pub struct LaserBlaster; 
impl ShipClass for LaserBlaster {}

/// Bundle used to spawn a new laser blaster
#[derive(Bundle)]
pub struct LaserBlasterBundle {
    pub identification: ShipIdentificationBundle<Legion, LaserBlaster>,
    pub smart_unit_bundle: SmartUnitBundle<ShootAction>,
    pub sprite_bundle: SpriteBundle,
    pub capabilities: ShipCapabilities<LaserProjectile>,
}

impl LaserBlasterBundle {
    pub fn new(transform: Transform, asset_server: &Res<AssetServer>) -> Self {
        Self {
            identification: ShipIdentificationBundle::default(),
            smart_unit_bundle: SmartUnitBundle::default(),
            capabilities: ShipCapabilities {
                can_shoot: CanShoot::new(Timer::from_seconds(1.0, TimerMode::Repeating)),
                can_translate: CanTranslate::from_coords_per_seconds(300.0),
                can_rotate: CanRotate::from_degrees_per_seconds(360.0),
            },
            sprite_bundle: SpriteBundle {
                transform,
                texture: asset_server.load("hostiles/laser.png"),
                ..default()
            },
        }
    }
}

