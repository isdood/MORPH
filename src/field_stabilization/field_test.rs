// Field Stabilization Test
use morph::field_stabilization::{FieldStabilizer, StabilizationIntegrator};
use morph::core::tensor::MorphicTensor;
use nalgebra::DVector;

fn main() {
    println!("Testing Field Stabilization...");

    // Create test tensor with potential field
    let mut tensor = MorphicTensor::void();
    tensor.potential.values = DVector::from_vec(vec![2.5, 1.8, 3.2]);

    // Create stabilizer
    let stabilizer = FieldStabilizer::new(0.1, 0.01, 100);
    let integrator = StabilizationIntegrator::new(stabilizer);

    // Run integration
    integrator.integrate(&mut tensor);

    println!("Final potential values: {:?}", tensor.potential.values.as_slice());
    println!("✅ Field stabilization tests completed!");
}
