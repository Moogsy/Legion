use bevy::prelude::*;

/// Marks an entity as having it's behavior controlled by a state machine
/// The transition states should be handled by the entity's module
#[derive(Component, Default)]
pub struct SmartUnitMarker;

#[derive(Bundle, Default)]
pub struct SmartUnitBundle<T: Component> {
    marker: SmartUnitMarker,
    default_state: T
}

#[derive(Component, Default)]
pub struct HaltState;