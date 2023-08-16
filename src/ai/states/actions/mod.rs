use bevy::prelude::*; 

mod rotate;
pub use rotate::{CanRotate, RotateAction};

mod shoot;
pub use shoot::{CanShoot, ShootAction};

mod translate;
pub use translate::{CanTranslate, TranslateAction};


/// This plugin contains logic for basic actions that can be performed by units
/// Those can be used directly, as if they were `AIState`, but using them is discouraged
pub struct ActionsPlugin;


impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(rotate::RotatePlugin)
            .add_plugins(shoot::ShootPlugin)
            .add_plugins(translate::TranslatePlugin);        
    }
}