use bevy::prelude::*; 

mod friendly;
mod hostile;

mod components;
pub use components::*; 

mod bundles;
pub use bundles::*;


pub struct SpaceshipsPlugin;

impl Plugin for SpaceshipsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(friendly::FriendlyPlugin)
            .add_plugins(hostile::HostilePlugin);
    }
}