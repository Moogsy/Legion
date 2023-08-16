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

mod actions;
pub use actions::*;

mod shoot_at;
pub use shoot_at::ShootAtState;

mod aim;
pub use aim::*;

/// This plugins contains logic used to implement a state machine for units
pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(actions::ActionsPlugin)
            .add_plugins(aim::AimPlugin)
            .add_plugins(shoot_at::ShootAtPlugin);
    }
}
