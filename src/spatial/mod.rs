/// Contains spatial trackers
use bevy::prelude::*;
use kiddo::KdTree;
pub use kiddo::distance::squared_euclidean;

mod ressources;
pub use ressources::*;

use crate::{spaceships::{Humanity, Legion}, projectiles::Bullet};

mod systems;

type KdTree2 = KdTree<f32, 2>;

pub struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app 
            .init_resource::<Tree<Humanity>>()
            .add_systems(Update, systems::rebuild_tree::<Humanity>)

            .init_resource::<Tree<Legion>>()
            .add_systems(Update, systems::rebuild_tree::<Legion>)
            
            .init_resource::<Tree<Bullet>>()
            .add_systems(Update, systems::rebuild_tree::<Bullet>);
    }
}