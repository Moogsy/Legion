use bevy::prelude::*;

/// Module containing states for ships and basic control for those states
mod capabilities;
pub use capabilities::*;

mod states;
pub use states::*;

mod descriptors;
pub use descriptors::*; 

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(StatesPlugin);
    }
}