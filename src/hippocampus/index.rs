// Hippocampal spatial indexing with spatial entanglement
#![allow(dead_code)]

use kiddo::{KdTree, SquaredEuclidean};
use crate::core::tensor::MorphicTensor;

pub struct CognitiveMap {
    pub tree: KdTree<f64, 2>,  // 2-dimensional tree
    counter: u64,  // Use u64 to match KdTree's item type
}

impl CognitiveMap {
    pub fn new() -> Self {
        CognitiveMap {
            tree: KdTree::new(),
            counter: 0,
        }
    }

    pub fn insert(&mut self, tensor: &MorphicTensor) {
        let point = tensor.position();
        self.tree.add(&point, self.counter);
        self.counter += 1;
    }

    pub fn nearest_neighbor(&self, point: [f64; 2]) -> Option<usize> {
        // Specify SquaredEuclidean distance metric explicitly
        let result = self.tree.nearest_one::<SquaredEuclidean>(&point);
        Some(result.item as usize)
    }

    pub fn size(&self) -> usize {
        self.tree.size() as usize
    }

    /// New: Spatial entanglement between positions
    pub fn spatial_entanglement(&self, pos1: [f64; 2], pos2: [f64; 2]) -> f64 {
        // Calculate entanglement strength based on distance
        let dx = pos1[0] - pos2[0];
        let dy = pos1[1] - pos2[1];
        let distance_sq = dx*dx + dy*dy;

        // Inverse square law for entanglement strength
        if distance_sq < f64::EPSILON {
            1.0
        } else {
            1.0 / distance_sq
        }
    }
}
