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
