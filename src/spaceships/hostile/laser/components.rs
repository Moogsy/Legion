use bevy::prelude::*;

use crate::{spaceships::Legion, ai::{SmartUnitMarker, HaltState}}; 

/// Enemy that shoots laser blasts
#[derive(Component, Default)]
pub struct LaserBlaster; 

/// Bundle used to spawn a new laser blaster
#[derive(Bundle, Default)]
pub struct LaserBlasterBundle {
    affiliation: Legion,
    class: LaserBlaster,
    smart_unit_marker: SmartUnitMarker,
    initial_state: HaltState,
}