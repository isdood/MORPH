// Field Stabilization Test
use morph::field_stabilization::FieldStabilizer;
use morph::core::tensor::MorphicTensor;
use nalgebra::DVector;

fn main() {
    println!("Testing Field Stabilization...");

    // Create test tensor with potential field
    let mut tensor = MorphicTensor::void();
    tensor.potential.values = DVector::from_vec(vec![2.5, 1.8, 3.2]);

    // Create stabilizer
    let stabilizer = FieldStabilizer::new(0.1, 0.01, 100);

    // Run stabilization
    stabilizer.gradient_descent(&mut tensor);
    let energy = stabilizer.developmental_energy(&tensor);

    println!("Developmental energy: {:.4}", energy);
    println!("✅ Field stabilization tests completed!");
}
