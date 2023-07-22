/// Contains spatial trackers
use bevy::prelude::*;
use kiddo::KdTree;

mod ressources;
pub use ressources::*;

use crate::spaceships::{Humanity, Legion};

mod systems;

type KdTree2 = KdTree<f32, 2>;

pub struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app 
            .init_resource::<TrackedByTree<Humanity>>()
            .add_systems(Update, systems::rebuild_tree::<Humanity>)

            .init_resource::<TrackedByTree<Legion>>()
            .add_systems(Update, systems::rebuild_tree::<Legion>);
    }
}