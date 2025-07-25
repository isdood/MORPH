// Evolutionary Optimization Test
use morph::evolutionary::EvolutionaryOptimizer;
use morph::core::tensor::MorphicTensor;
use morph::field_stabilization::FieldStabilizer;
use nalgebra::DVector;
use rand::Rng;

fn main() {
    println!("Testing Evolutionary Optimization...");

    // Create test tensor
    let mut tensor = MorphicTensor::void();
    tensor.entanglement.strength = 0.5;
    tensor.potential.values = DVector::from_vec(vec![1.5, 0.8, 2.2, 1.0]);

    // Create field stabilizer
    let mut stabilizer = FieldStabilizer::new(0.1, 0.01, 100);

    // Create optimizer
    let mut optimizer = EvolutionaryOptimizer::new();

    // Simulate performance metric
    let mut rng = rand::thread_rng();

    // Run multiple optimization cycles
    for i in 0..5 {
        println!("\n--- Optimization Cycle {} ---", i + 1);
        let performance = rng.gen_range(0.6..1.0);
        optimizer.optimize(&mut tensor, &mut stabilizer, performance);
    }

    println!("Final entanglement strength: {:.4}", tensor.entanglement.strength);
    println!("Final learning rate: {:.6}", stabilizer.learning_rate);
    println!("Final quantum threshold: {:.4}",
             optimizer.feedback_loop.hybrid_computation.quantum_threshold);

    println!("✅ Evolutionary optimization tests completed!");
}
