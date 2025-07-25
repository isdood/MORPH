// Phylogenetic Runtime Test
use morph::phylogenetic::{PhylogeneticRuntime, DeltaApplicator, EnvironmentalSelector, QuantumForker};
use morph::core::tensor::MorphicTensor;

fn main() {
    println!("Testing Phylogenetic Runtime...");

    // Create runtime
    let runtime = PhylogeneticRuntime::new(0.8);

    // Create test tensor
    let mut tensor = MorphicTensor::void();
    tensor.entanglement.strength = 0.7;

    // Test delta application
    let delta_app = DeltaApplicator::new(runtime);
    delta_app.apply_delta(&mut tensor, &[42]);
    println!("Applied delta: {:?}", tensor.temporal.versions);

    // Test environmental selection
    let selector = EnvironmentalSelector::new(PhylogeneticRuntime::new(0.6));
    let tensors = vec![
        MorphicTensor::void(),
        MorphicTensor::void(),
        tensor.clone()
    ];
    let selected = selector.select(&tensors);
    println!("Selected indices: {:?}", selected);

    // Test quantum forking
    let forker = QuantumForker::new(PhylogeneticRuntime::new(0.7));
    let fork = forker.fork_tensor(&tensor);
    println!("Fork created with path: {:?}", fork.temporal.versions);

    println!("✅ Runtime test completed!");
}
