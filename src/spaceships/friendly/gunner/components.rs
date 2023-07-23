use bevy::prelude::*;

use crate::{
    ai::{SmartUnitBundle, ShootState},
    spaceships::Humanity, projectiles::Bullet,
};

#[derive(Component, Default)]
pub struct Gunner; 

/// Bundle used to spawn a new gunner
#[derive(Bundle, Default)]
pub struct GunnerBundle {
    pub affiliation: Humanity,
    pub class: Gunner,
    pub projectile_type: Bullet,
    pub smart_unit_bundle: SmartUnitBundle<ShootState>,
    pub sprite_bundle: SpriteBundle,
}