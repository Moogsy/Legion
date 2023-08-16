use bevy::prelude::*;

use super::{Affliliation, ShipClass}; 

#[derive(Bundle, Default)]
pub struct ShipIdentificationBundle<T, U>
where
    T: Affliliation,
    U: ShipClass, 
{
    pub affiliation: T,
    pub class: U,
}

use crate::{ai::{CanShoot, CanTranslate, CanRotate}, projectiles::ProjectileType}; 

#[derive(Bundle, Default)]
pub struct ShipCapabilities<T: ProjectileType> {
    pub can_shoot: CanShoot<T>,
    pub can_translate: CanTranslate,
    pub can_rotate: CanRotate,
}