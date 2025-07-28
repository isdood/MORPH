//! Environment interaction module for reinforcement learning
#![allow(dead_code)]

use ndarray::Array1;

/// Trait defining the interface for RL environments
pub trait Environment {
    /// Reset the environment to its initial state
    fn reset(&mut self) -> Array1<f64>;
    
    /// Take an action in the environment
    fn step(&mut self, action: usize) -> (Array1<f64>, f64, bool);
    
    /// Get the current state
    fn get_state(&self) -> &Array1<f64>;
    
    /// Get the number of possible actions
    fn action_space_size(&self) -> usize;
    
    /// Get the state space size
    fn state_space_size(&self) -> usize;
}

/// Simple test environment for verification
pub struct TestEnvironment {
    state: Array1<f64>,
    steps: usize,
    max_steps: usize,
}

impl TestEnvironment {
    pub fn new(state_size: usize, max_steps: usize) -> Self {
        TestEnvironment {
            state: Array1::zeros(state_size),
            steps: 0,
            max_steps,
        }
    }
}

impl Environment for TestEnvironment {
    fn reset(&mut self) -> Array1<f64> {
        self.state = Array1::zeros(self.state.len());
        self.steps = 0;
        self.state.clone()
    }
    
    fn step(&mut self, action: usize) -> (Array1<f64>, f64, bool) {
        self.steps += 1;
        
        // Simple dynamics: move towards the action index
        let mut reward = -0.1; // Small negative reward per step
        
        if action < self.state.len() {
            self.state[action] += 0.1;
            reward += 1.0 / (action as f64 + 1.0);
        }
        
        let done = self.steps >= self.max_steps;
        (self.state.clone(), reward, done)
    }
    
    fn get_state(&self) -> &Array1<f64> {
        &self.state
    }
    
    fn action_space_size(&self) -> usize {
        4 // Fixed action space for testing
    }
    
    fn state_space_size(&self) -> usize {
        self.state.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_environment_reset() {
        let mut env = TestEnvironment::new(3, 10);
        let state = env.reset();
        assert_eq!(state, Array1::<f64>::zeros(3));
    }
    
    #[test]
    fn test_environment_step() {
        let mut env = TestEnvironment::new(3, 10);
        env.reset();
        let (next_state, reward, done) = env.step(1);
        assert_eq!(next_state[1], 0.1);
        assert!(reward > 0.0);
        assert!(!done);
    }
}
