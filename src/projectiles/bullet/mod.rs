use bevy::prelude::*;

mod components;
pub use components::Bullet;

mod systems;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, systems::shoot);
    }
}