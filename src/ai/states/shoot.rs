use bevy::prelude::*;

use crate::{ai::SmartUnitMarker, projectiles::{ProjectileType, ShootEvent}};


/// State when the concerned entity will try to shoot forward
#[derive(Component, Default)]
pub struct ShootState;

/// Creates a new ShootEvent to be handled by the projectiles mod
pub fn shoot<T: Component + ProjectileType>(
    query: Query<(&Handle<Image>, &Transform), (With<SmartUnitMarker>, With<ShootState>, With<T>)>,
    mut writer: EventWriter<ShootEvent<T>>,
    assets: Res<Assets<Image>>,
) {
    for (handle, transform) in query.iter() {
        let image = assets.get(handle).unwrap();
        let size = image.texture_descriptor.size;

        writer.send(
            ShootEvent::<T>::in_front_of(transform, &size)
        )
    }
}