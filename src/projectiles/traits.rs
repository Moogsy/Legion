/// Contains markers telling what kind of projectiles the concerned entity can shoot
use bevy::prelude::*;


pub trait ProjectileType: Component + Default {}
