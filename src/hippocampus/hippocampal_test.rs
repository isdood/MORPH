// Hippocampal Architecture Test
use morph::hippocampus::index::CognitiveMap;
use morph::hippocampus::pattern_completion::PatternCompleter;
use morph::hippocampus::oscillation::Oscillator;
use morph::hippocampus::memory::QuantumMemoryManager;
use morph::core::tensor::MorphicTensor;

fn main() {
    println!("Testing Hippocampal Architecture...");

    // Create cognitive map
    let mut cognitive_map = CognitiveMap::new();

    // Create and insert sample tensors
    let mut tensor1 = MorphicTensor::void();
    tensor1.spatial.coordinates = [1.0, 2.0];
    cognitive_map.insert(&tensor1);

    let mut tensor2 = MorphicTensor::void();
    tensor2.spatial.coordinates = [3.0, 4.0];
    cognitive_map.insert(&tensor2);

    println!("Cognitive map contains {} tensors", cognitive_map.size());

    // Test pattern completion
    let pattern_completer = PatternCompleter::new(cognitive_map, 0.5);
    let partial_point = [1.1, 2.1];
    if let Some(id) = pattern_completer.complete_pattern(partial_point) {
        println!("Pattern completed for point {:?} with ID: {}", partial_point, id);
    }

    // Test theta-gamma oscillation
    let oscillator = Oscillator::new(4.0, 40.0, 1.0);
    oscillator.simulate_cycle();

    // Test spatial entanglement
    let entanglement_strength = pattern_completer.cognitive_map.spatial_entanglement(
        [1.0, 2.0], [3.0, 4.0]
    );
    println!("Spatial entanglement strength: {}", entanglement_strength);

    // Test quantum memory management
    let memory_manager = QuantumMemoryManager::new(100);
    memory_manager.store(&tensor1);
    memory_manager.retrieve(0);

    println!("✅ Hippocampal tests completed!");
}
