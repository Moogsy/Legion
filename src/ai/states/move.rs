use std::marker::PhantomData;

/// Moves an entity forward at the given velocity
use bevy::prelude::*;

use crate::{measurement_units::TranslationSpeed, ai::DirectionDescriptor};


#[derive(Component, Default)]
pub struct MoveState<T: DirectionDescriptor> {
    pub speed: TranslationSpeed,
    _direction: PhantomData<T>,
}


