use bevy::prelude::*; 

mod projectile_types;
pub use projectile_types::*; 

mod events;
pub use events::*; 

mod bullet;
pub use bullet::Bullet; 

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(bullet::BulletPlugin);
    }
}