// System Manifestation Test
use morph::system_manifestation::{KernelInterface, RealityRenderer, ObserverMediator, ConsciousnessAPI};
use morph::core::tensor::MorphicTensor;
use morph::quantum::state::QuantumState;
use nalgebra::DVector;

fn main() {
    println!("Testing System Manifestation...");

    // Create test tensors with potential values
    let mut tensor1 = MorphicTensor::void();
    tensor1.quantum_state = QuantumState::Collapsed;
    tensor1.entanglement.strength = 0.8;
    tensor1.spatial.coordinates = [0.4, 0.6];
    tensor1.potential.values = DVector::from_vec(vec![1.2, 0.5, 2.1]);

    let mut tensor2 = MorphicTensor::void();
    tensor2.entanglement.strength = 0.6;
    tensor2.spatial.coordinates = [0.7, 0.3];
    tensor2.potential.values = DVector::from_vec(vec![0.8, 1.5, 0.9]);

    // Test Kernel Interface
    let kernel = KernelInterface::new();
    kernel.handle_syscall(&tensor1);

    // Test Reality Rendering
    let renderer = RealityRenderer::new();
    let reality = renderer.render_reality_tunnel(&[tensor1.clone(), tensor2.clone()]);
    println!("Reality tunnel: {:?}", reality);

    // Test Observer Mediation
    let mediator = ObserverMediator::new();
    let mediated = mediator.mediate_perspective(&reality);
    println!("Mediated perspective: {:?}", mediated);

    // Test Consciousness API
    let api = ConsciousnessAPI::new();
    let (eeg, phi) = api.get_metrics(&[tensor1.clone(), tensor2.clone()]);
    println!("Consciousness metrics - φ: {:.4}", phi);
    println!("EEG Readings:");
    for (band, amp) in eeg {
        println!("  {}: {:.2} μV", band, amp);
    }

    println!("✅ System manifestation tests completed!");
}
