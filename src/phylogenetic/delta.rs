// Developmental delta applicator
#![allow(dead_code)]

use crate::core::tensor::MorphicTensor;
use crate::phylogenetic::PhylogeneticRuntime;

pub struct DeltaApplicator {
    pub runtime: PhylogeneticRuntime,
}

impl DeltaApplicator {
    pub fn new(runtime: PhylogeneticRuntime) -> Self {
        DeltaApplicator { runtime }
    }

    pub fn apply_delta(&self, tensor: &mut MorphicTensor, delta: &[usize]) {
        self.runtime.apply_mutation_in_superposition(tensor, delta);
        self.runtime.collapse_dead_live(tensor);
    }
}
