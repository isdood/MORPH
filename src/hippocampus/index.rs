// Hippocampal spatial indexing
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
        // Use SquaredEuclidean distance metric
        let result = self.tree.nearest_one::<SquaredEuclidean>(&point);
        Some(result.item as usize)
    }

    pub fn size(&self) -> usize {
        self.tree.size() as usize
    }
}
