// Quantum forking implementation
#![allow(dead_code)]

use crate::core::tensor::MorphicTensor;
use crate::phylogenetic::PhylogeneticRuntime;

pub struct QuantumForker {
    pub runtime: PhylogeneticRuntime,
}

impl QuantumForker {
    pub fn new(runtime: PhylogeneticRuntime) -> Self {
        QuantumForker { runtime }
    }

    pub fn fork_tensor(&self, original: &MorphicTensor) -> MorphicTensor {
        self.runtime.quantum_fork(original)
    }

    pub fn bulk_fork(&self, originals: &[MorphicTensor]) -> Vec<MorphicTensor> {
        originals.iter().map(|t| self.fork_tensor(t)).collect()
    }
}
