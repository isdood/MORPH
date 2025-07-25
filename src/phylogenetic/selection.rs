// Environmental selection mechanics
#![allow(dead_code)]

use crate::core::tensor::MorphicTensor;
use crate::phylogenetic::PhylogeneticRuntime;

pub struct EnvironmentalSelector {
    pub runtime: PhylogeneticRuntime,
}

impl EnvironmentalSelector {
    pub fn new(runtime: PhylogeneticRuntime) -> Self {
        EnvironmentalSelector { runtime }
    }

    pub fn select(&self, tensors: &[MorphicTensor]) -> Vec<usize> {
        tensors.iter()
            .enumerate()
            .filter(|(_, t)| self.runtime.apply_selection_pressure(t))
            .map(|(i, _)| i)
            .collect()
    }
}
