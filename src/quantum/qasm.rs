// Quantum Assembly (QASM) operations
#![allow(dead_code)]

#[derive(Debug, Clone)]
pub enum QuantumOperation {
    H, // Hadamard gate
    X, // Pauli-X gate
    Y, // Pauli-Y gate
    Z, // Pauli-Z gate
    CX, // Controlled-X (CNOT) gate
    T, // T gate
    Measure, // Measurement operation
    Custom(String), // Custom operation
}

impl QuantumOperation {
    pub fn to_qasm(&self) -> String {
        match self {
            QuantumOperation::H => "h".to_string(),
            QuantumOperation::X => "x".to_string(),
            QuantumOperation::Y => "y".to_string(),
            QuantumOperation::Z => "z".to_string(),
            QuantumOperation::CX => "cx".to_string(),
            QuantumOperation::T => "t".to_string(),
            QuantumOperation::Measure => "measure".to_string(),
            QuantumOperation::Custom(op) => op.clone(),
        }
    }
}
