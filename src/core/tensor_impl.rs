// Morphic Tensor Implementation Test
use morph::core::tensor::MorphicTensor;
use morph::quantum::state::QuantumSystem;

fn main() {
    println!("Testing Morphic Tensor...");

    // Create and test void tensor
    let mut void_tensor = MorphicTensor::void();
    println!("Void tensor created successfully! State: {:?}", void_tensor.quantum_state);

    // Test phylogenetic delta
    void_tensor.apply_phylogenetic_delta(&[1, 2, 3]);
    println!("Applied phylogenetic delta: {:?}", void_tensor.temporal.versions);

    // Test quantum fork
    let forked_tensor = void_tensor.quantum_fork();
    println!("Forked tensor state: {:?}", forked_tensor.quantum_state);

    // Test entanglement
    let other_tensor = MorphicTensor::void();  // Removed unnecessary 'mut'
    void_tensor.entangle_with(&other_tensor);
    println!("Entanglement strength: {}", void_tensor.entanglement.strength);

    // Test collapse
    void_tensor.collapse();
    println!("Collapsed state: {:?}", void_tensor.quantum_state);

    // Test quantum system
    let quantum_system = QuantumSystem::new();
    let new_state = quantum_system.collapse_state(forked_tensor.quantum_state);
    println!("Quantum system collapsed state: {:?}", new_state);

    // Test hippocampal indexing
    let mut cognitive_map = morph::hippocampus::index::CognitiveMap::new();
    cognitive_map.insert(&void_tensor);
    cognitive_map.insert(&forked_tensor);
    println!("Cognitive map contains {} tensors", cognitive_map.size());

    // Test nearest neighbor search
    let point = [0.0, 0.0];
    if let Some(id) = cognitive_map.nearest_neighbor(point) {
        println!("Nearest tensor to {:?} has ID: {}", point, id);
    }
}
