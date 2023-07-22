/// Contains markers telling what kind of projectiles the concerned entity can shoot
use bevy::prelude::*;


pub trait ProjectileType {}



/// Marks this unit as a laser shooter
#[derive(Component, Default)]
pub struct Laser;
