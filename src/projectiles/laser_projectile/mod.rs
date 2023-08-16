use bevy::prelude::*;

mod components;
pub use components::*;

use super::ShootEvent; 

mod systems;

pub struct LaserProjectilePlugin;

impl Plugin for LaserProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShootEvent<LaserProjectile>>()
            .add_systems(Update, systems::shoot);
    }
}