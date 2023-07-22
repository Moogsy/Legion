use bevy::prelude::*; 

mod components;
pub use components::*; 

mod gunner;

pub struct FriendlyPlugin;

impl Plugin for FriendlyPlugin {
    fn build(&self, app: &mut App) {
        
    }
}