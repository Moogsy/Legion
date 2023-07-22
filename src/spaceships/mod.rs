use bevy::prelude::*; 

mod friendly;
mod hostile;

mod components;
pub use components::*; 


pub struct SpaceshipsPlugin;

impl Plugin for SpaceshipsPlugin {
    fn build(&self, app: &mut App) {
        
    }
}