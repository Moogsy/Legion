use bevy::prelude::*;

mod components;
pub use components::Bullet;

use super::ShootEvent;

mod systems;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShootEvent<Bullet>>()
            .add_systems(Update, systems::shoot);
    }
}