// Bidirectional compiler main module with fixes
#![allow(dead_code)]

mod top_down;
mod bottom_up;
mod adjoint_invariance;

use crate::core::tensor::MorphicTensor;
pub use top_down::TopDownCompiler;
pub use bottom_up::BottomUpCompiler;
pub use adjoint_invariance::AdjointInvarianceChecker;

pub struct BidirectionalCompiler {
    pub top_down: TopDownCompiler,
    pub bottom_up: BottomUpCompiler,
    pub adjoint_checker: AdjointInvarianceChecker,
}

impl BidirectionalCompiler {
    pub fn new() -> Self {
        BidirectionalCompiler {
            top_down: TopDownCompiler::new(),
            bottom_up: BottomUpCompiler::new(0.7),
            adjoint_checker: AdjointInvarianceChecker::new(1e-5),
        }
    }

    pub fn compile(&self, tensors: &mut [MorphicTensor]) {
        println!("Bidirectional compilation starting...");
        self.top_down.compile(tensors);
        // Additional compilation logic would go here
    }
}
