use bevy::prelude::*;

use crate::{measurement_units::RotationSpeed, ai::SmartUnitMarker}; 

pub struct RotatePlugin;

impl Plugin for RotatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, rotate);
    }
}

#[derive(Component, Default)]
pub struct CanRotate(pub RotationSpeed);

impl CanRotate {
    pub fn from_degrees_per_seconds(deg: f32) -> Self {
        Self(RotationSpeed::from_degrees_per_seconds(deg))
    }
}

#[derive(Default, Clone, Copy)]
pub enum RotationDirection {
    #[default]
    Clockwise = -1,
    CounterClockwise = 1,
}

#[derive(Component, Default)]
pub struct RotateAction {
    pub direction: RotationDirection,
}

impl RotateAction {
    pub fn clockwise() -> Self {
        Self { direction: RotationDirection::Clockwise }
    }

    pub fn counter_clockwise() -> Self {
        Self { direction: RotationDirection::CounterClockwise }
    }

}

fn rotate(
    mut query: Query<(&mut Transform, &CanRotate, &RotateAction), With<SmartUnitMarker>>,
    time: Res<Time>,
) {
    query.for_each_mut(|(mut transform, rotate, rotate_action)| {
        transform.rotate_z(rotate.0.speed() * time.delta_seconds() * rotate_action.direction as i32 as f32);
    });
}