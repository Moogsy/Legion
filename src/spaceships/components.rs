use bevy::prelude::*;

/// Marks a struct as one that describes in which camp the ship belongs to
pub trait Affliliation: Component + Default {}

/// Component that can be used as a ship class
pub trait ShipClass: Component {}


/// Marker for allied ships
#[derive(Component, Default)]
pub struct Humanity;
impl Affliliation for Humanity {}

/// Marker for enemy ships
#[derive(Component, Default)]
pub struct Legion;
impl Affliliation for Legion {}

