use std::marker::PhantomData;

use  bevy::{prelude::*, render::render_resource::Extent3d};

use super::ProjectileType; 

#[derive(Event)]
pub struct ShootEvent<T: Component + ProjectileType> {
    pub location: Vec2,
    pub orientation: Quat,
    _projectile_type: PhantomData<T>
}

impl<T: Component + ProjectileType> ShootEvent<T> {
    pub fn in_front_of(transform: &Transform, size: &Extent3d) -> Self {
        let shooter_rotation = (transform.rotation * Vec3::Y).truncate();
        let bullet_starting_point = transform.translation.truncate() + shooter_rotation * (size.height as f32) / 2.0;

        Self {
            location: bullet_starting_point,
            orientation: transform.rotation,
            _projectile_type: PhantomData
        }
    }
}