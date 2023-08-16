use bevy::prelude::*;

use crate::{projectiles::{ProjectileType, ShootEvent}, ai::{SmartUnitBundle, TranslateAction}, measurement_units::TranslationSpeed}; 

/// Marks this unit as a laser shooter
#[derive(Component, Default)]
pub struct LaserProjectile;
impl ProjectileType for LaserProjectile {}

#[derive(Bundle, Default)]
pub struct LaserProjectileBundle {
    pub projectile_type: LaserProjectile,
    pub smart_unit_bundle: SmartUnitBundle<TranslateAction>,
    pub sprite_bundle: SpriteBundle,
}

impl LaserProjectileBundle {
    pub fn new(event: &ShootEvent<LaserProjectile>, asset_server: &Res<AssetServer>) -> Self {
        let location = event.location;
        let orientation = event.orientation;

        Self { 
            projectile_type: LaserProjectile, 
            sprite_bundle: SpriteBundle { 
                texture: asset_server.load("projectiles/laser.png"),
                transform: Transform {
                    translation: location.extend(0.0),
                    rotation: orientation,
                    ..default()
                },
                ..default()
            },
            smart_unit_bundle: SmartUnitBundle::with_default_state(
                TranslateAction::horizontal(TranslationSpeed::from_coords_per_seconds(800.0))
            )
        }
    }
}
