#!/bin/bash
# Phase 18: Quantum Learning Environment & Basic Training Loop

set -e

# Ensure we're in the project root
cd "$(dirname "$0")/.."

echo "🚀 Starting Phase 18: Quantum Learning Environment & Basic Training Loop"

# Create demo directory structure
mkdir -p src/demo

# Create quantum learning environment
cat > src/demo/quantum_env.rs << 'EOL'
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
EOL

# Create training module
cat > src/demo/training.rs << 'EOL'
//! Training loop for quantum learning

use morph::learning::QRLAgent;
use ndarray::{Array1, Array2};
use std::time::Duration;

/// Train a QRL agent on a quantum environment
pub fn train_quantum_agent(
    agent: &mut QRLAgent,
    episodes: usize,
    state_size: usize,
    max_steps: usize,
    render_every: usize,
) -> Vec<f64> {
    let mut env = super::quantum_env::QuantumLearningEnv::new(state_size, max_steps);
    let mut rewards = Vec::with_capacity(episodes);
    
    for episode in 0..episodes {
        let mut state = env.reset();
        let mut total_reward = 0.0;
        let mut done = false;
        
        while !done {
            let action = agent.select_action(&state);
            let (next_state, reward, episode_done) = env.step(action);
            
            // Train the agent
            let mut states = Array2::zeros((1, state_size));
            let mut actions = vec![action];
            let mut rewards_vec = vec![reward];
            
            agent.update_policy(&states, &actions, &rewards_vec);
            
            state = next_state;
            total_reward += reward;
            done = episode_done;
            
            // Render occasionally
            if episode % render_every == 0 {
                env.render();
                println!("Episode: {}, Step: {}, Reward: {:.3}", 
                       episode, env.steps, reward);
                std::thread::sleep(Duration::from_millis(100));
            }
        }
        
        rewards.push(total_reward);
        if episode % 10 == 0 {
            println!("Episode: {}, Total Reward: {:.3}", episode, total_reward);
        }
    }
    
    rewards
}
EOL

# Create demo module
cat > src/demo/mod.rs << 'EOL'
//! Quantum learning demonstration

pub mod quantum_env;
pub mod training;

/// Run a simple quantum learning demonstration
pub fn run_demo() {
    println!("🚀 Starting Quantum Learning Demo");
    
    use morph::learning::QRLAgent;
    use training::train_quantum_agent;
    
    // Initialize agent and environment
    let state_size = 4;
    let action_size = 4;
    let learning_rate = 0.01;
    let gamma = 0.99;
    
    println!("Initializing QRL Agent...");
    let mut agent = QRLAgent::new(state_size, action_size, learning_rate, gamma);
    
    // Training parameters
    let episodes = 1000;
    let max_steps = 100;
    let render_every = 100;  // Render every N episodes
    
    println!("Training agent for {} episodes...", episodes);
    let start_time = std::time::Instant::now();
    
    let rewards = train_quantum_agent(
        &mut agent, 
        episodes, 
        state_size, 
        max_steps, 
        render_every
    );
    
    let duration = start_time.elapsed();
    let avg_reward = rewards.iter().sum::<f64>() / rewards.len() as f64;
    
    println!("\n🎉 Training complete!");
    println!("Time elapsed: {:.2?}", duration);
    println!("Average reward: {:.3}", avg_reward);
    
    // Show final performance
    if let Some((min, max)) = rewards.iter().min_by(|a, b| a.partial_cmp(b).unwrap())
        .and_then(|min| rewards.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).map(|max| (min, max))) {
        println!("Reward range: [{:.3}, {:.3}]", min, max);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_demo_components() {
        // Simple test to verify components can be imported
        let _env = quantum_env::QuantumLearningEnv::new(3, 10);
        assert!(true);
    }
}
EOL

# Update main.rs to include and run the demo
cat > src/main.rs << 'EOL'
// MORPH Kernel Entry Point

mod demo;

fn main() {
    println!("MORPH Kernel v0.1 - Genesis Initiated");
    
    // Run the quantum learning demo
    demo::run_demo();
}
EOL

# Add necessary dependencies if not present
if ! grep -q "rand" Cargo.toml; then
    sed -i '/^\[dependencies\]/a rand = "0.8"' Cargo.toml
fi

# Clean up the code to fix warnings
cat > src/demo/training.rs << 'EOL'
//! Training loop for quantum learning

use morph::learning::QRLAgent;
use ndarray::Array2;

/// Train a QRL agent on a quantum environment
pub fn train_quantum_agent(
    agent: &mut QRLAgent,
    episodes: usize,
    state_size: usize,
    max_steps: usize,
    render_every: usize,
) -> Vec<f64> {
    let mut env = super::quantum_env::QuantumLearningEnv::new(state_size, max_steps);
    let mut rewards = Vec::with_capacity(episodes);
    
    for episode in 0..episodes {
        let mut state = env.reset();
        let mut total_reward = 0.0;
        let mut done = false;
        
        while !done {
            let action = agent.select_action(&state);
            let (next_state, reward, episode_done) = env.step(action);
            
            // Train the agent
            let states = Array2::zeros((1, state_size));
            let actions = vec![action];
            let rewards_vec = vec![reward];
            
            agent.update_policy(&states, &actions, &rewards_vec);
            
            state = next_state;
            total_reward += reward;
            done = episode_done;
            
            // Render occasionally
            if episode % render_every == 0 {
                env.render();
                println!("Episode: {}, Step: {}, Reward: {:.3}", 
                       episode, env.steps, reward);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
        
        rewards.push(total_reward);
        if episode % 10 == 0 {
            println!("Episode: {}, Total Reward: {:.3}", episode, total_reward);
        }
    }
    
    rewards
}
EOL

# Add necessary dependencies if not present
if ! grep -q "rand" Cargo.toml; then
    sed -i '/^\[dependencies\]/a rand = "0.8"' Cargo.toml
fi

echo "✅ Phase 18 completed successfully!"
echo "Run 'cargo run --bin morph' to see the quantum learning demo in action!"
echo "This will train a quantum learning agent and show its progress..."
