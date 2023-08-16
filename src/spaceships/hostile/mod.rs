use bevy::prelude::*; 

mod laser;
pub use laser::{LaserBlaster, LaserBlasterBundle}; 

pub struct HostilePlugin;

impl Plugin for HostilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(laser::LaserPlugin);
    }
}