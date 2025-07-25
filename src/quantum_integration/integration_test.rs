// Quantum Integration Test
use morph::quantum_integration::{QuantumStateIntegrator, EntanglementProtocol};
use morph::core::tensor::MorphicTensor;
use morph::quantum::qasm::QuantumOperation;
use morph::quantum::state::QuantumState;
use morph::hippocampus::memory::QuantumMemoryManager;  // Added missing import

fn main() {
    println!("Testing Quantum Integration...");

    // Create quantum integrator
    let mut integrator = QuantumStateIntegrator::new(0.7);

    // Create entanglement protocol
    let protocol = EntanglementProtocol::new(
        1.5,
        vec![
            QuantumOperation::H,
            QuantumOperation::CX,
            QuantumOperation::Custom("rz(π/4)".to_string())
        ]
    );
    integrator.register_protocol("hyper_entangle", protocol);

    // Create test tensors
    let mut tensor1 = MorphicTensor::void();
    let mut tensor2 = MorphicTensor::void();

    // Test state transitions
    integrator.void_to_superposition(&mut tensor1);
    println!("Tensor1 state: {:?}", tensor1.quantum_state);

    // Test observation mechanics
    integrator.apply_observation(&mut tensor1, 0.9);
    println!("Tensor1 state after observation: {:?}", tensor1.quantum_state);

    // Reset tensor1 to superposition
    tensor1.quantum_state = QuantumState::Superposition;

    // Test entanglement protocol
    integrator.execute_entanglement_protocol("hyper_entangle", &mut tensor1, &mut tensor2);
    println!("Tensor1 entanglement strength: {}", tensor1.entanglement.strength);
    println!("Tensor2 entanglement strength: {}", tensor2.entanglement.strength);

    // Test QASM generation
    let operations = vec![
        QuantumOperation::H,
        QuantumOperation::CX,
        QuantumOperation::Measure,
        QuantumOperation::Custom("custom_gate 3.14".to_string())
    ];
    let qasm_code = integrator.generate_qasm(&operations);
    println!("Generated QASM code:\n{}", qasm_code);

    // Test memory storage with Void state
    let memory = QuantumMemoryManager::new(100);
    memory.store(&tensor1);
    memory.store(&MorphicTensor::void());

    println!("✅ Quantum integration tests completed!");
}
