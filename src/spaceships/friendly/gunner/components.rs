use bevy::prelude::*;

use crate::{
    ai::{SmartUnitBundle, CanShoot, CanTranslate, CanRotate, ShootAtState, Nearest},
    projectiles::Bullet,
    spaceships::{Humanity, ShipIdentificationBundle, ShipClass, ShipCapabilities, Legion}, 
};

#[derive(Component, Default)]
pub struct Gunner; 
impl ShipClass for Gunner {}

/// Bundle used to spawn a new gunner
#[derive(Bundle)]
pub struct GunnerBundle {
    pub identification: ShipIdentificationBundle<Humanity, Gunner>,
    pub smart_unit_bundle: SmartUnitBundle<ShootAtState<Nearest, Legion>>,
    pub sprite_bundle: SpriteBundle,
    pub capabilities: ShipCapabilities<Bullet>,
}

impl GunnerBundle {
    pub fn new(transform: Transform, asset_server: &Res<AssetServer>) -> Self {
        Self {
            identification: ShipIdentificationBundle::default(),
            smart_unit_bundle: SmartUnitBundle::default(),
            capabilities: ShipCapabilities {
                can_shoot: CanShoot::<Bullet>::new(Timer::from_seconds(1.0, TimerMode::Repeating)),
                can_translate: CanTranslate::from_coords_per_seconds(300.0),
                can_rotate: CanRotate::from_degrees_per_seconds(360.0),
            },
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("friendlies/gunner.png"),
                transform,
                ..default()
            },
        }
    }
}