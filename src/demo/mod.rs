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
