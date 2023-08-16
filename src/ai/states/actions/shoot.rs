use bevy::prelude::*; 

use crate::{
    ai::{
        LogicState,
        SmartUnitMarker,
    },
    projectiles::{ShootEvent, ProjectileType, Bullet, LaserProjectile},
};

pub struct ShootPlugin;

impl Plugin for ShootPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (shoot::<Bullet>, update_cooldowns::<Bullet>).chain())
            .add_systems(Update, (shoot::<LaserProjectile>, update_cooldowns::<LaserProjectile>).chain());
    }
}

#[derive(Component, Default)]
pub struct CanShoot<T: ProjectileType> {
    pub projectile_type: T,
    pub cooldown: Timer,
}

impl <T: ProjectileType> CanShoot<T> {
    pub fn new(cooldown: Timer) -> CanShoot<T> {
        CanShoot { projectile_type: T::default(), cooldown }
    }
}


/// Units in this state will systematically shoot forward
#[derive(Component, Default)]
pub struct ShootAction;
impl LogicState for ShootAction {}

/// Updates shooting timer
fn update_cooldowns<T: ProjectileType>(mut query: Query<&mut CanShoot<T>>, time: Res<Time>) {
    query.for_each_mut(|mut can_shoot|  {
        can_shoot.cooldown.tick(time.delta());
    });
}

/// Spawns a new bullet everytime this is called
fn shoot<T: ProjectileType>(
    mut query: Query<(&Handle<Image>, &Transform, &mut CanShoot<T>), (With<SmartUnitMarker>, With<ShootAction>)>,
    mut writer: EventWriter<ShootEvent<T>>,
    assets: Res<Assets<Image>>,
) {
    query.for_each_mut(|(handle, transform, mut can_shoot)| {
        if !can_shoot.cooldown.just_finished() {
            return;
        }

        let image = match assets.get(handle) {
            Some(image) => image,
            // If the texture isn't properly loaded yet, 
            // try shooting again next frame
            // Thought this shouldn't happen
            None => return can_shoot.cooldown.reset(),
        };

        let size = image.texture_descriptor.size;

        writer.send(
            ShootEvent::<T>::in_front_of(transform, &size)
        );
    });
}
