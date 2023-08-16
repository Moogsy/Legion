use std::marker::PhantomData;

use bevy::prelude::*; 
use crate::{ai::{TargetLocation, Nearest, Furthest, LogicState}, spaceships::{Affliliation, Humanity, Legion}, spatial::Tree, projectiles::{ProjectileType, Bullet, LaserProjectile}};

use super::AimState; 

pub struct ShootAtPlugin; 

impl Plugin for ShootAtPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, 
            (
                shoot_at::<Nearest, Humanity>,
                shoot_at::<Nearest, Legion>,

                shoot_at::<Furthest, Humanity>,
                shoot_at::<Furthest, Legion>,
            )
        );
    }
}

#[derive(Component, Default)]
pub struct ShootAtState<T: TargetLocation, U: Affliliation> {
    target_location: PhantomData<T>,
    target_camp: PhantomData<U>,
}

impl<T: TargetLocation, U: Affliliation> LogicState for ShootAtState<T, U> {}

fn shoot_at<T: TargetLocation, U: Affliliation>(
    mut commands: Commands,
    tree: Res<Tree<U>>,
    mut shooters_query: Query<(Entity, &Transform, Option<&mut AimState>), (With<ShootAtState<T, U>>, Without<U>)>,
    targets_query: Query<&Transform, With<U>>,
) {
    // No one to target, no need to do anything
    if tree.kdtree.size() == 0 {
        return;
    }

    shooters_query.for_each_mut(|(shooter_entity, shooter_transform, maybe_aimstate)| {
        let coords = [shooter_transform.translation.x, shooter_transform.translation.y];

        let (_dist, entity_id) = tree.kdtree.nearest_one(&coords, &T::dist_func);

        let target_entity = Entity::from_bits(entity_id as u64);
        let Ok(target_transform) = targets_query.get(target_entity) else {return};

        match maybe_aimstate {
            Some(mut aim_state) => {aim_state.target_location = target_transform.translation.truncate() },
            None => {
                let mut entity_commands = commands.entity(shooter_entity);
                entity_commands.insert(AimState::new(target_transform.translation.truncate()));
            }
        };
    });
}