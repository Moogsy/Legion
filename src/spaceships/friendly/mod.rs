use bevy::prelude::*; 
mod gunner;

pub struct FriendlyPlugin;

impl Plugin for FriendlyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(gunner::GunnerPlugin);
    }
}