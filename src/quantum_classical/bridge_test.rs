// Quantum-Classical Bridge Test
use morph::quantum_classical::{HybridComputation, DecoherenceHandler, ClassicalShadowing};
use morph::core::tensor::MorphicTensor;
use morph::quantum::qasm::QuantumOperation;
use nalgebra::DVector;

fn main() {
    println!("Testing Quantum-Classical Bridge...");

    // Create test tensor
    let mut tensor = MorphicTensor::void();
    tensor.entanglement.strength = 0.6;
    tensor.potential.values = DVector::from_vec(vec![1.2, 0.8, 2.4, 0.9, 1.7]);

    // Test Hybrid Computation
    let hybrid = HybridComputation::new(0.5);
    let operations = vec![
        QuantumOperation::H,
        QuantumOperation::CX,
        QuantumOperation::Measure
    ];
    hybrid.execute(&mut tensor, &operations);

    // Test Decoherence Handling
    let decoherence_handler = DecoherenceHandler::new();
    for _ in 0..5 {
        decoherence_handler.handle_decoherence(&mut tensor);
    }

    // Test Classical Shadowing
    let shadowing = ClassicalShadowing::new(100);
    let shadow = shadowing.create_shadow(&tensor);
    println!("Created classical shadow with {} samples", shadow.len());

    if !shadow.is_empty() {
        let reconstruction = shadowing.reconstruct(&shadow);
        println!("Shadow reconstruction: {:.4}", reconstruction);

        let actual_avg = tensor.potential.values.mean();
        println!("Actual average: {:.4}", actual_avg);
        println!("Error: {:.4}%", (reconstruction - actual_avg).abs() / actual_avg * 100.0);
    }

    println!("✅ Quantum-classical bridge tests completed!");
}
