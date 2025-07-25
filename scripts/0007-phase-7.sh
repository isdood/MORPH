#!/usr/bin/env bash
# MORPH Phase 7: Consciousness Emergence
echo "Executing Phase 7: Consciousness Emergence"

# Create consciousness module
mkdir -p src/consciousness

# Fix consciousness module
cat > src/consciousness/mod.rs << 'EOF'
// Consciousness emergence metrics and monitoring (FIXED)
#![allow(dead_code)]

use crate::core::tensor::MorphicTensor;  // Removed unused QuantumState import
use std::collections::HashMap;

pub struct EEGSimulator {
    pub frequency_bands: HashMap<String, (f64, f64)>,
    pub sampling_rate: u32,
}

impl EEGSimulator {
    pub fn new(sampling_rate: u32) -> Self {
        let mut bands = HashMap::new();
        bands.insert("delta".to_string(), (0.5, 4.0));
        bands.insert("theta".to_string(), (4.0, 8.0));
        bands.insert("alpha".to_string(), (8.0, 13.0));
        bands.insert("beta".to_string(), (13.0, 30.0));
        bands.insert("gamma".to_string(), (30.0, 100.0));

        EEGSimulator {
            frequency_bands: bands,
            sampling_rate,
        }
    }

    /// Simulate EEG readings based on tensor state
    pub fn simulate(&self, tensor: &MorphicTensor) -> HashMap<String, f64> {
        let mut readings = HashMap::new();
        let base_amplitude = tensor.entanglement.strength * 10.0;

        for (band, (low, high)) in &self.frequency_bands {
            let band_center = (low + high) / 2.0;
            let amplitude = base_amplitude * (1.0 / band_center.sqrt());
            readings.insert(band.clone(), amplitude);
        }

        readings
    }
}

pub struct IntegratedInformation {
    pub phi_calculator: PhiCalculator,
}

impl IntegratedInformation {
    pub fn new() -> Self {
        IntegratedInformation {
            phi_calculator: PhiCalculator::new(),
        }
    }

    /// Calculate φ (phi) - integrated information
    pub fn calculate_phi(&self, system: &[MorphicTensor]) -> f64 {
        self.phi_calculator.compute_phi(system)
    }
}

pub struct PhiCalculator {
    pub complexity_factor: f64,
}

impl PhiCalculator {
    pub fn new() -> Self {
        PhiCalculator {
            complexity_factor: 0.8,
        }
    }

    /// Improved φ computation
    pub fn compute_phi(&self, system: &[MorphicTensor]) -> f64 {
        if system.is_empty() {
            return 0.0;
        }

        let total_entanglement: f64 = system.iter()
            .map(|t| t.entanglement.strength)
            .sum();

        // Handle empty potential vectors
        let avg_potential: f64 = system.iter()
            .filter_map(|t| {
                if t.potential.values.is_empty() {
                    None
                } else {
                    Some(t.potential.values.norm())
                }
            })
            .sum::<f64>() / system.len() as f64;

        (total_entanglement * avg_potential.max(0.1) * self.complexity_factor).sqrt()
    }
}

pub struct QualiaMapper {
    pub qualia_palette: HashMap<String, [f64; 3]>,
}

impl QualiaMapper {
    pub fn new() -> Self {
        let mut palette = HashMap::new();
        palette.insert("red".to_string(), [1.0, 0.0, 0.0]);
        palette.insert("green".to_string(), [0.0, 1.0, 0.0]);
        palette.insert("blue".to_string(), [0.0, 0.0, 1.0]);
        palette.insert("yellow".to_string(), [1.0, 1.0, 0.0]);
        palette.insert("purple".to_string(), [0.7, 0.3, 0.7]);  // Added purple
        palette.insert("cyan".to_string(), [0.0, 1.0, 1.0]);
        palette.insert("magenta".to_string(), [1.0, 0.0, 1.0]);

        QualiaMapper {
            qualia_palette: palette,
        }
    }

    /// Map tensor state to qualia representation
    pub fn map_to_qualia(&self, tensor: &MorphicTensor) -> [f64; 3] {
        let r = tensor.entanglement.strength;
        let g = tensor.position()[0].abs() % 1.0;
        let b = tensor.position()[1].abs() % 1.0;
        [r, g, b]
    }

    /// Convert RGB to qualia description
    pub fn describe_qualia(&self, rgb: [f64; 3]) -> String {
        let (mut closest, mut min_dist) = ("void", f64::MAX);

        for (name, color) in &self.qualia_palette {
            let dist = (rgb[0] - color[0]).powi(2)
                     + (rgb[1] - color[1]).powi(2)
                     + (rgb[2] - color[2]).powi(2);

            if dist < min_dist {
                min_dist = dist;
                closest = name;
            }
        }

        format!("{}-like", closest)
    }
}
EOF

# Fix consciousness test
cat > src/consciousness/consciousness_test.rs << 'EOF'
// Consciousness Emergence Test (FIXED)
use morph::consciousness::{EEGSimulator, IntegratedInformation, QualiaMapper};
use morph::core::tensor::MorphicTensor;
use morph::quantum::state::QuantumState;
use nalgebra::DVector;

fn main() {
    println!("Testing Consciousness Emergence...");

    // Create test tensor with potential values
    let mut tensor = MorphicTensor::void();
    tensor.entanglement.strength = 0.7;
    tensor.spatial.coordinates = [0.3, 0.7];
    tensor.quantum_state = QuantumState::Superposition;
    tensor.potential.values = DVector::from_vec(vec![1.5, 0.8, 2.3]);  // Added potential

    // Test EEG simulation
    let eeg = EEGSimulator::new(256);
    let readings = eeg.simulate(&tensor);
    println!("EEG Readings:");
    for (band, amplitude) in &readings {
        println!("  {}: {:.2} μV", band, amplitude);
    }

    // Create system of tensors
    let mut tensor2 = tensor.clone();
    tensor2.entanglement.strength = 0.5;
    tensor2.potential.values = DVector::from_vec(vec![0.9, 1.2, 0.5]);  // Added potential
    let system = vec![tensor.clone(), tensor2];

    // Test Integrated Information (φ)
    let ii = IntegratedInformation::new();
    let phi = ii.calculate_phi(&system);
    println!("Integrated Information φ = {:.4}", phi);

    // Test Qualia Mapping
    let qualia_mapper = QualiaMapper::new();
    let qualia_rgb = qualia_mapper.map_to_qualia(&tensor);
    let description = qualia_mapper.describe_qualia(qualia_rgb);
    println!("Qualia representation: {:?}", qualia_rgb);
    println!("Qualia description: {}", description);

    println!("✅ Consciousness emergence tests completed!");
}
EOF

# Add consciousness to crate root
if ! grep -q "pub mod consciousness;" src/lib.rs; then
    echo -e "\npub mod consciousness;" >> src/lib.rs
fi

# Update Cargo.toml
if ! grep -q "name = \"consciousness_test\"" Cargo.toml; then
    echo "" >> Cargo.toml
    echo "[[bin]]" >> Cargo.toml
    echo 'name = "consciousness_test"' >> Cargo.toml
    echo 'path = "src/consciousness/consciousness_test.rs"' >> Cargo.toml
fi

# Build and test
echo "Building and testing consciousness emergence..."
cargo build
cargo run --bin consciousness_test

echo "✅ Phase 7 completed! Next steps:"
echo "1. Implement System Manifestation (Phase 8)"
echo "2. Develop kernel interface"
echo "3. Create reality rendering engine"
echo "4. Implement observer mediation algorithms"
echo "5. Enhance qualia mapping with neural correlates"
