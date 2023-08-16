use bevy::prelude::*;
use kiddo::KdTree;

use super::{Tree, KdTree2};

pub fn rebuild_tree<T: Component>(
    query: Query<(Entity, &Transform), With<T>>,
    mut spatial_tracker: ResMut<Tree<T>>,
) {

    let mut new_tree: KdTree2 = KdTree::new();

    for (entity, transform) in query.iter() {
        new_tree.add(
            &[transform.translation.x, transform.translation.y], 
            // Might cause issues on 32 bits targets
            // The current implementation on Bevy is putting
            // the generation as the first 32 bits
            // and the id as the last 32 bits
            entity.to_bits() as usize
        );
    }
    spatial_tracker.kdtree = new_tree;
}