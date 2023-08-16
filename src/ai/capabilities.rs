use bevy::prelude::*;

/// Marks this entity as one that has an AI
/// It's presence will be systematically checked before making it interact
/// with anything
#[derive(Component, Default)]
pub struct SmartUnitMarker;



pub trait LogicState: Component + Default {}


#[derive(Bundle, Default)]
pub struct SmartUnitBundle<T: LogicState> {
    pub marker: SmartUnitMarker,
    pub default_state: T
}

impl<T: LogicState> SmartUnitBundle<T> {

    pub fn with_default_state(default_state: T) -> Self {
        Self {
            marker: SmartUnitMarker,
            default_state,
        }
    }


    pub fn new() -> Self {
        Self { marker: SmartUnitMarker, default_state: T::default() }
    }
}

#[derive(Component, Default)]
pub struct HaltState;
impl LogicState for HaltState {}
