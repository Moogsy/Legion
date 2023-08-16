use bevy::prelude::*;

use crate::{measurement_units::TranslationSpeed, ai::{SmartUnitMarker, LogicState}};

pub struct TranslatePlugin; 

impl Plugin for TranslatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, translate);
    }
}

#[derive(Component, Default)]
pub struct CanTranslate(pub TranslationSpeed);

impl CanTranslate {
    pub fn from_coords_per_seconds(coords: f32) -> Self {
        Self(TranslationSpeed::from_coords_per_seconds(coords))
    }
}


/// Units in this state will translate forward
/// at the given speed.
/// If the speed is negative, the unit will go backwards
#[derive(Component, Default)]
pub struct TranslateAction {
    // Positive horizontal will make it go forward, inversly for negative
    pub horizontal: TranslationSpeed, 
    // Positive vertical will make it go on it's right, inversly for negatgive
    pub vertical: TranslationSpeed,
}

impl TranslateAction {
    pub fn horizontal(speed: TranslationSpeed) -> Self {
        Self {
            horizontal: speed,
            vertical: TranslationSpeed::zero(),
        }
    }
}
impl LogicState for TranslateAction {}

pub fn translate(
    mut query: Query<(&mut Transform, &TranslateAction), With<SmartUnitMarker>>,
    time: Res<Time>,
) {
    for (mut transform, translate_state) in query.iter_mut() {
        let vertical_movement_direction = transform.rotation * Vec3::Y;
        let vertical_movement_distance = translate_state.horizontal.speed() * time.delta_seconds();

        transform.translation += vertical_movement_direction * vertical_movement_distance;

        let horizontal_movement_direction = transform.rotation * Vec3::X;
        let horizontal_movement_distance= translate_state.vertical.speed() * time.delta_seconds();
        
        transform.translation += horizontal_movement_direction * horizontal_movement_distance;
    }
}

