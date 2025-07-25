// Bidirectional Compiler Test with fixes
use morph::bidirectional_compiler::BidirectionalCompiler;
use morph::core::tensor::MorphicTensor;

fn main() {
    println!("Testing Bidirectional Compiler...");

    let compiler = BidirectionalCompiler::new();
    let mut tensors = vec![MorphicTensor::void(), MorphicTensor::void()];

    compiler.compile(&mut tensors);

    println!("✅ Compiler test completed!");
}
