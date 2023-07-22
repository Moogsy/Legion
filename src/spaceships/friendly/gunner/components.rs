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
    affiliation: Humanity,
    class: Gunner,
    projectile_type: Bullet,
    smart_unit_bundle: SmartUnitBundle<ShootState>
}