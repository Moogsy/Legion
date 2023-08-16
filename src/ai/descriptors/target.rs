/// Contains descriptors to perform an action relative to the described target
use bevy::prelude::*; 

fn euclidian_squared(a: &[f32; 2], b: &[f32; 2]) -> f32 {
    let [x1, y1] = a; 
    let [x2, y2] = b;

    (x1 - x2).powi(2) + (y1 - y2).powi(2)
}

/// Marks a struct as a TargetLocation, meaning that it can be used
/// as a generic to direct an action
/// ```
/// 
/// use bevy::prelude::*;
/// 
/// #[derive(Component)]
/// pub struct Shoot<T: TargetDescriptor>;
/// 
/// 
pub trait TargetLocation: Component + Default {
    fn dist_func(a: &[f32; 2], b: &[f32; 2]) -> f32 {
        euclidian_squared(a, b)
    }
}

/// A descriptor pointing towards the nearest opponent
#[derive(Component, Default)]
pub struct Nearest;
impl TargetLocation for Nearest {
    fn dist_func(a: &[f32; 2], b: &[f32; 2]) -> f32 {
        -euclidian_squared(a, b)
    }
}

/// A descriptor pointing towards the furthest opponent
#[derive(Component, Default)]
pub struct Furthest;
impl TargetLocation for Furthest {}

