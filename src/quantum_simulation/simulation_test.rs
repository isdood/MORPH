// Quantum Simulation Test
use morph::quantum_simulation::{QuantumSimulator, DistributedSimulator};
use morph::core::tensor::MorphicTensor;
use morph::quantum::qasm::QuantumOperation;
use morph::quantum::state::QuantumState;
use nalgebra::DVector;

fn main() {
    println!("Testing Quantum Simulation Layer...");

    // Create test tensor
    let mut tensor = MorphicTensor::void();
    tensor.quantum_state = QuantumState::Superposition;
    tensor.entanglement.strength = 0.75;
    tensor.potential.values = DVector::from_vec(vec![1.2, 0.8, 2.4]);

    // Test single-node simulator
    let simulator = QuantumSimulator::new(2);
    let mut state = simulator.initialize_state(&tensor);
    println!("Initial state: {:?}", state.iter().map(|c| c.norm_sqr()).collect::<Vec<f64>>());

    // Apply Hadamard to qubit 0
    simulator.apply_gate(&mut state, &QuantumOperation::H, 0, None);
    println!("After H(0): {:?}", state.iter().map(|c| c.norm_sqr()).collect::<Vec<f64>>());

    // Apply CNOT between qubit 0 and 1
    simulator.apply_gate(&mut state, &QuantumOperation::CX, 1, Some(0));
    println!("After CX(0,1): {:?}", state.iter().map(|c| c.norm_sqr()).collect::<Vec<f64>>());

    // Apply T gate to qubit 1
    simulator.apply_gate(&mut state, &QuantumOperation::T, 1, None);
    println!("After T(1): {:?}", state.iter().map(|c| c.norm_sqr()).collect::<Vec<f64>>());

    // Measure the system
    let result = simulator.measure(&mut state);
    println!("Measurement result: {}", result);
    println!("Collapsed state: {:?}", state.iter().map(|c| c.norm_sqr()).collect::<Vec<f64>>());

    // Test distributed simulator
    let cluster = DistributedSimulator::new(4, 3);
    let operations = vec![
        QuantumOperation::H,
        QuantumOperation::H,
        QuantumOperation::CX,
        QuantumOperation::T,
        QuantumOperation::Measure
    ];
    cluster.simulate_distributed(&tensor, &operations);

    println!("✅ Quantum simulation tests completed!");
}
