// Hardware Integration Test
use morph::hardware::{MockQuantumProcessor, QuantumBackend, HybridScheduler, QuantumHardware};
use morph::core::tensor::MorphicTensor;
use morph::quantum::qasm::QuantumOperation;

fn main() {
    println!("Testing Hardware Integration...");

    // Create mock quantum processor
    let mut mock_hw = MockQuantumProcessor::new();

    // Test calibration
    mock_hw.calibrate();
    mock_hw.calibrate();
    println!("Calibration count: {}", mock_hw.calibration_count);

    // Test error profile
    let error_profile = mock_hw.error_rates();
    println!("Hardware error profile:");
    println!("T1: {:?}, T2: {:?}", error_profile.t1_time, error_profile.t2_time);
    println!("Gate fidelity: {:.4}, Readout fidelity: {:.4}",
             error_profile.gate_fidelity, error_profile.readout_fidelity);

    // Create quantum backend
    let backend = QuantumBackend::Hardware(Box::new(mock_hw));

    // Create scheduler
    let scheduler = HybridScheduler::new(backend, 3);

    // Create test tensor and operations
    let mut tensor = MorphicTensor::void();
    tensor.entanglement.strength = 0.75; // Set initial entanglement strength
    let operations = vec![
        QuantumOperation::H,
        QuantumOperation::X,
        QuantumOperation::CX,
        QuantumOperation::Measure,
    ];

    // Test hybrid scheduling (above threshold)
    scheduler.schedule_execution(&operations, &mut tensor);

    // Test hybrid scheduling (below threshold)
    let small_ops = vec![QuantumOperation::H, QuantumOperation::Measure];
    scheduler.schedule_execution(&small_ops, &mut tensor);

    // Test error mitigation
    let original_strength = tensor.entanglement.strength;
    scheduler.apply_error_mitigation(&mut tensor);
    println!("Entanglement strength: {:.2} -> {:.2}",
             original_strength, tensor.entanglement.strength);

    println!("✅ Hardware integration tests completed!");
}
