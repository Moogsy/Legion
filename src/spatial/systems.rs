use bevy::prelude::*;
use kiddo::KdTree;

use super::{TrackedByTree, KdTree2};

pub fn rebuild_tree<T: Component>(
    query: Query<(Entity, &Transform), With<T>>,
    mut spatial_tracker: ResMut<TrackedByTree<T>>,
) {

    let mut new_tree: KdTree2 = KdTree::new();

    for (entity, transform) in query.iter() {
        new_tree.add(
            &[transform.translation.x, transform.translation.y], 
            entity.to_bits() as usize
        );
    }
    spatial_tracker.kdtree = new_tree;
}