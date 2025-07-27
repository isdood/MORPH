// Consciousness Metrics Test
use morph::consciousness_metrics::{IntegratedInformation, NeuralOscillationTracker, ConsciousnessMonitor};
use morph::core::tensor::MorphicTensor;
use morph::quantum::state::QuantumState;
use nalgebra::DVector;

fn main() {
    println!("Testing Consciousness Metrics...");

    // Create test tensors
    let mut tensor1 = MorphicTensor::void();
    tensor1.quantum_state = QuantumState::Superposition;
    tensor1.entanglement.strength = 0.8;
    tensor1.potential.values = DVector::from_vec(vec![1.5, 0.9, 2.1]);

    let mut tensor2 = MorphicTensor::void();
    tensor2.entanglement.strength = 0.6;
    tensor2.potential.values = DVector::from_vec(vec![0.7, 1.2, 0.8]);

    let system = vec![tensor1, tensor2];

    // Test Integrated Information
    let phi_calculator = IntegratedInformation::new();
    let phi = phi_calculator.calculate_phi(&system);
    println!("System Φ: {:.4}", phi);
    println!("Is conscious? {}", phi_calculator.is_conscious(phi));

    // Test Neural Oscillation Tracking
    let tracker = NeuralOscillationTracker::new();
    let oscillations = tracker.track_oscillations(&system);
    println!("Neural oscillations: {:?}", oscillations);

    // Test Consciousness Monitor
    let monitor = ConsciousnessMonitor::new();
    monitor.monitor(&system);

    println!("✅ Consciousness metrics tests completed!");
}
