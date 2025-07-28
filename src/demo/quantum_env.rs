//! Simple quantum learning environment

use ndarray::Array1;
use rand::Rng;

/// Quantum learning environment that evolves a quantum state
pub struct QuantumLearningEnv {
    pub target_state: Array1<f64>,
    pub current_state: Array1<f64>,
    pub steps: usize,
    max_steps: usize,
    state_size: usize,
}

impl QuantumLearningEnv {
    /// Create a new quantum learning environment
    pub fn new(state_size: usize, max_steps: usize) -> Self {
        let mut rng = rand::thread_rng();
        
        // Create random target state (normalized)
        let mut target = Array1::zeros(state_size);
        for x in target.iter_mut() {
            *x = rng.gen_range(-1.0..1.0);
        }
        let dot_product: f64 = target.dot(&target);
        let norm = dot_product.sqrt().max(f64::EPSILON);
        target /= norm;
        
        QuantumLearningEnv {
            target_state: target.clone(),
            current_state: Array1::zeros(state_size),
            steps: 0,
            max_steps,
            state_size,
        }
    }
    
    /// Reset the environment to initial state
    pub fn reset(&mut self) -> Array1<f64> {
        self.current_state = Array1::zeros(self.state_size);
        self.steps = 0;
        self.current_state.clone()
    }
    
    /// Take a step in the environment
    pub fn step(&mut self, action: usize) -> (Array1<f64>, f64, bool) {
        let mut rng = rand::thread_rng();
        
        // Simple quantum-inspired state evolution
        let perturbation = 0.1;
        let mut new_state = self.current_state.clone();
        
        // Apply action (simplified for demonstration)
        if action < self.state_size {
            new_state[action] += rng.gen_range(-perturbation..perturbation);
        }
        
        // Normalize
        let norm = (new_state.dot(&new_state)).sqrt();
        new_state /= norm;
        
        // Calculate reward (negative distance to target)
        let mut reward = -self.distance_to_target(&new_state);
        
        // Small penalty for taking too many steps
        reward -= 0.01;
        
        self.current_state = new_state;
        self.steps += 1;
        
        let done = self.steps >= self.max_steps;
        
        (self.current_state.clone(), reward, done)
    }
    
    /// Calculate distance to target state
    fn distance_to_target(&self, state: &Array1<f64>) -> f64 {
        // Calculate L2 distance to target
        let diff = &self.target_state - state;
        diff.dot(&diff).sqrt()
    }
    
    /// Render the current state
    pub fn render(&self) {
        println!("\n=== Quantum Learning Environment ===");
        println!("Step: {}/{}", self.steps, self.max_steps);
        println!("Current State: {:.*}", 3, self.current_state);
        println!("Target State:  {:.*}", 3, self.target_state);
        println!("Distance: {:.3}", self.distance_to_target(&self.current_state));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_env_initialization() {
        let env = QuantumLearningEnv::new(4, 100);
        assert_eq!(env.current_state.len(), 4);
        assert_eq!(env.target_state.len(), 4);
    }
    
    #[test]
    fn test_env_step() {
        let mut env = QuantumLearningEnv::new(3, 10);
        let (state, reward, done) = env.step(0);
        assert_eq!(state.len(), 3);
        assert!(!done);
    }
}
