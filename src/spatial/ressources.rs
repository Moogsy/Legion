use std::marker::PhantomData;

use bevy::prelude::*; 
use kiddo::KdTree;

use super::KdTree2;

/// Ressource that will update the kdtree on each frame with all entities containing marker T
#[derive(Resource)]
pub struct Tree<T: Component> {
    pub kdtree: KdTree<f32, 2>,
    _tracked_components: PhantomData<T>,
}

impl<T: Component> Default for Tree<T> {
    fn default() -> Self {
        let kdtree: KdTree2 = KdTree::new();

        Self { kdtree, _tracked_components: PhantomData }
    }
}
