/// File used to implements States for `Spaceships`
/// ```
/// use bevy::prelude::*; 
/// 
/// #[derive(Component)]
/// struct Cat;
/// #[derive(Component)]
/// struct Eat; 
/// #[derive(Component)]
/// struct Sleep;
/// #[derive(Bundle)]
/// struct CatStateBundle {
///   sleep: Sleep,
///   eat: Eat,
/// }
/// fn spawn(commands: &mut Commands) { commands.spawn((Cat, Sleep)); }
/// 
/// fn update_sleeping_cats(query: Query<Cat, With<Sleep>) { /* snip */ }
/// 
/// fn update_eating_cats(query: Query<Cat, With<Eat>) { /* snip */ }
/// 
/// fn make_satiated_cats_sleep(mut commands: Commands, query: Query<Entity, (With<Cat>, With<Eat>)>) {  
///     for cat in query.iter() {
///       let mut entity_commands = commands.entity(cat)
///       entity_commands.remove::<CatStateBundle>();
///       entity_commands.insert(Sleep);
///     }
/// }
/// ```

use bevy::prelude::*;

mod shoot;
pub use shoot::*;

mod r#move;
pub use r#move::*;

/// Marks a structure as one that can be used as a state for an unit
pub trait UnitState: Component {}

#[derive(Bundle)]
pub struct StateCleanupBundle {
    pub shoot: ShootState,
}
