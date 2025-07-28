#!/bin/bash
# Phase 17: Tensor Integration Implementation

set -e

# Ensure we're in the project root
cd "$(dirname "$0")/.."

echo "🚀 Starting Phase 17: Tensor Integration Implementation"

# Create a backup of the original tensor file
cp src/core/tensor.rs src/core/tensor.rs.bak

# Update the tensor implementation with integration methods
cat > src/core/tensor.rs << 'EOL'
// Morphic Tensor: Fundamental 5D quantum structure
#![allow(dead_code)]

use crate::learning::QNNLayer;
use crate::quantum::state::QuantumState;
use ndarray::{Array1, Array2};
use nalgebra::DVector;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SpatialStructure {
    pub coordinates: [f64; 2],  // Using 2D coordinates for simplicity
    pub data: Option<Array1<f64>>, // Tensor data storage
}

#[derive(Debug, Clone)]
pub struct PhylogeneticPath {
    pub versions: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct EntanglementField {
    pub connections: Vec<usize>,
    pub strength: f64,
}

#[derive(Debug, Clone)]
pub struct MorphicGradient {
    pub values: DVector<f64>,
}

#[derive(Debug, Clone)]
pub struct ObserverPerspective {
    pub weights: [f64; 2],
}

#[derive(Debug, Clone)]
pub struct MorphicTensor {
    pub spatial: SpatialStructure,
    pub temporal: PhylogeneticPath,
    pub entanglement: EntanglementField,
    pub potential: MorphicGradient,
    pub observer: ObserverPerspective,
    pub quantum_state: QuantumState,
}

impl Default for SpatialStructure {
    fn default() -> Self {
        SpatialStructure {
            coordinates: [0.0, 0.0],
            data: None,
        }
    }
}

impl MorphicTensor {
    /// Creates a new tensor in void state
    pub fn void() -> Self {
        MorphicTensor {
            spatial: SpatialStructure::default(),
            temporal: PhylogeneticPath { versions: Vec::new() },
            entanglement: EntanglementField { connections: Vec::new(), strength: 0.0 },
            potential: MorphicGradient { values: DVector::zeros(0) },
            observer: ObserverPerspective { weights: [1.0, 1.0] },
            quantum_state: QuantumState::default(),
        }
    }

    /// Creates a new tensor with the specified data
    pub fn from_data(data: Array1<f64>) -> Self {
        let mut tensor = Self::void();
        tensor.spatial.data = Some(data);
        tensor
    }

    /// Applies a QNN layer to the tensor
    pub fn apply_qnn_layer(&self, layer: &QNNLayer) -> Self {
        if let Some(data) = &self.spatial.data {
            let output = layer.forward(data);
            Self::from_data(output)
        } else {
            // If no data, return a copy of self
            self.clone()
        }
    }
    
    /// Converts the tensor data to an Array1<f64>
    pub fn to_array(&self) -> Array1<f64> {
        self.spatial.data.clone().unwrap_or_else(|| Array1::zeros(0))
    }
    
    /// Updates the tensor data from a slice
    pub fn update_from_slice(&mut self, data: &[f64]) {
        self.spatial.data = Some(Array1::from_vec(data.to_vec()));
    }
    
    /// Creates a batch of tensors from a 2D array
    pub fn batch_from_array2(array: Array2<f64>) -> Vec<Self> {
        array.outer_iter()
            .map(|row| Self::from_data(row.to_owned()))
            .collect()
    }
    
    /// Applies a function element-wise to the tensor data
    pub fn map<F>(&self, f: F) -> Self 
    where
        F: Fn(f64) -> f64,
    {
        if let Some(data) = &self.spatial.data {
            let new_data = data.mapv(f);
            Self::from_data(new_data)
        } else {
            self.clone()
        }
    }
    
    /// Computes the dot product with another tensor
    pub fn dot(&self, other: &Self) -> f64 {
        if let (Some(a), Some(b)) = (&self.spatial.data, &other.spatial.data) {
            a.dot(b)
        } else {
            0.0
        }
    }
    
    /// Computes the L2 norm of the tensor data
    pub fn norm(&self) -> f64 {
        if let Some(data) = &self.spatial.data {
            data.dot(data).sqrt()
        } else {
            0.0
        }
    }
    
    /// Normalizes the tensor data
    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        if norm > 0.0 {
            self.map(|x| x / norm)
        } else {
            self.clone()
        }
    }
    
    // ... existing methods ...
    
    /// Applies a developmental delta to the tensor
    pub fn apply_phylogenetic_delta(&mut self, delta: &[usize]) {
        // Apply delta to the tensor data if it exists
        if let Some(data) = &mut self.spatial.data {
            for &idx in delta {
                if idx < data.len() {
                    data[idx] += 1.0; // Simple increment for demonstration
                }
            }
        }
    }
    
    /// Creates a quantum fork of the current tensor
    pub fn quantum_fork(&self) -> Self {
        let mut forked = self.clone();
        // Add a small random perturbation to the data
        if let Some(data) = &mut forked.spatial.data {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            *data = data.mapv(|x| x + rng.gen_range(-0.1..0.1));
        }
        forked
    }
    
    /// Entangles this tensor with another
    pub fn entangle_with(&mut self, other: &Self) {
        // Simple entanglement: average the data vectors
        if let (Some(self_data), Some(other_data)) = (&self.spatial.data, &other.spatial.data) {
            if self_data.len() == other_data.len() {
                let avg_data = (self_data + other_data) / 2.0;
                self.spatial.data = Some(avg_data);
            }
        }
        // Increase entanglement strength
        self.entanglement.strength = (self.entanglement.strength + 1.0).min(1.0);
    }
    
    /// Collapses the quantum state
    pub fn collapse(&mut self) {
        if let Some(data) = &mut self.spatial.data {
            // Simple collapse: round values to nearest integer
            *data = data.mapv(|x| x.round());
        }
        // Reset quantum state to collapsed
        self.quantum_state = QuantumState::Collapsed;
    }
    
    /// Get the spatial position as a 2D point
    pub fn position(&self) -> [f64; 2] {
        self.spatial.coordinates
    }
}

// Implement Display for better debugging
impl fmt::Display for MorphicTensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MorphicTensor {{\n  position: {:?},\n  data: {},\n  versions: {:?},\n  entanglement_strength: {:.2}\n}}",
            self.position(),
            if let Some(data) = &self.spatial.data {
                format!("Some({:?})", data.to_vec())
            } else {
                "None".to_string()
            },
            self.temporal.versions,
            self.entanglement.strength
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_tensor_creation() {
        let tensor = MorphicTensor::void();
        assert_eq!(tensor.position(), [0.0, 0.0]);
        assert!(tensor.spatial.data.is_none());
        
        let data = array![1.0, 2.0, 3.0];
        let tensor = MorphicTensor::from_data(data.clone());
        assert_eq!(tensor.to_array(), data);
    }
    
    #[test]
    fn test_tensor_operations() {
        let data = array![3.0, 4.0];
        let tensor = MorphicTensor::from_data(data);
        
        // Test norm
        assert_relative_eq!(tensor.norm(), 5.0, epsilon = 1e-10);
        
        // Test normalization
        let normalized = tensor.normalize();
        assert_relative_eq!(normalized.norm(), 1.0, epsilon = 1e-10);
        
        // Test dot product
        let other = MorphicTensor::from_data(array![1.0, 0.0]);
        assert_relative_eq!(tensor.dot(&other), 3.0, epsilon = 1e-10);
    }
    
    #[test]
    fn test_tensor_mapping() {
        let data = array![1.0, 2.0, 3.0];
        let tensor = MorphicTensor::from_data(data);
        
        // Test map
        let doubled = tensor.map(|x| x * 2.0);
        assert_eq!(doubled.to_array(), array![2.0, 4.0, 6.0]);
    }
    
    #[test]
    fn test_batch_creation() {
        let array = array![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];
        let batch = MorphicTensor::batch_from_array2(array);
        
        assert_eq!(batch.len(), 3);
        assert_eq!(batch[0].to_array(), array![1.0, 2.0]);
        assert_eq!(batch[1].to_array(), array![3.0, 4.0]);
        assert_eq!(batch[2].to_array(), array![5.0, 6.0]);
    }
}
EOL

# Add the approx crate for floating point comparisons in tests
if ! grep -q "approx" Cargo.toml; then
    sed -i '/^\[dependencies\]/a approx = "0.5"' Cargo.toml
fi

echo "✅ Phase 17 completed successfully!"
echo "The tensor module has been updated with integration methods for QNN layers and array operations."
