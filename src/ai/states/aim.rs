use bevy::prelude::*;

use crate::ai::LogicState;

use super::CanRotate;

pub struct AimPlugin;

impl Plugin for AimPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update,aim);

    }
}

#[derive(Component, Default)]
pub struct AimState {
    pub target_location: Vec2,
}

impl AimState {
    pub fn new(target_location: Vec2) -> Self {
        Self { target_location }
    }
}
impl LogicState for AimState {}

fn aim(
    mut shooters_query: Query<(&mut Transform, &CanRotate, &AimState)>,
    time: Res<Time>,
) {
    shooters_query.for_each_mut(|(mut shooter_transform, can_rotate, aim_state)| {
        let to_target = (shooter_transform.translation.truncate() - aim_state.target_location).normalize_or_zero();

        let shooter_right = (shooter_transform.rotation * Vec3::X).truncate();
        let right_dot_target = shooter_right.dot(to_target);

        let rotation_sign = f32::copysign(1.0, right_dot_target);
        let max_angle = right_dot_target.clamp(-1.0, 1.0).acos();

        let rotation_angle = rotation_sign * (can_rotate.0.speed() * time.delta_seconds()).min(max_angle);
        shooter_transform.rotate_z(rotation_angle);
    });
}