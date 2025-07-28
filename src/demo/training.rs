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
