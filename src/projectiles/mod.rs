use bevy::prelude::*; 

mod traits;
pub use traits::*; 

mod events;
pub use events::*; 

mod bullet;
pub use bullet::Bullet; 

mod laser_projectile;
pub use laser_projectile::LaserProjectile;

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(laser_projectile::LaserProjectilePlugin);
    }
}