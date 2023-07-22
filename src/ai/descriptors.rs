/// Contains descriptors to perform an action relative to the described target
use bevy::prelude::*; 

/// Marks a struct as a TargetDescriptor, meaning that it can be used
/// as a generic to direct an action
/// ```
/// 
/// use bevy::prelude::*;
/// 
/// #[derive(Component)]
/// pub struct Shoot<T: TargetDescriptor>;
/// 
/// 
pub trait TargetDescriptor {}

/// A descriptor pointing towards the nearest opponent
#[derive(Component, Default)]
pub struct Nearest;
impl TargetDescriptor for Nearest {}

/// A descriptor pointing towards the furthest opponent
#[derive(Component, Default)]
pub struct Furthest;
impl TargetDescriptor for Furthest {}


/// Marks a struct as a `DirectionDescriptor`, which 
/// describes the direction in which an action will be performed
/// relative to the ship's current transform.
pub trait DirectionDescriptor {}

#[derive(Component, Default)]
pub struct Forward;
impl DirectionDescriptor for Forward {}

#[derive(Component, Default)]
pub struct Backward;
impl DirectionDescriptor for Backward {}


#[derive(Component, Default)]
pub struct Left;
impl DirectionDescriptor for Left {}


#[derive(Component, Default)]
pub struct Right; 
impl DirectionDescriptor for Right {}