use bevy::prelude::*;

mod components;
pub use components::*;

mod systems;

pub struct GunnerPlugin;

impl Plugin for GunnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, systems::test_spawn);
    }
}