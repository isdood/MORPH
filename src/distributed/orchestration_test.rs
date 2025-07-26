// Distributed Orchestration Test
use morph::distributed::{NodeManager, TaskScheduler, ResultAggregator};
use morph::core::tensor::MorphicTensor;
use morph::quantum::qasm::QuantumOperation;

fn main() {
    println!("Testing Distributed Orchestration...");

    // Create node manager
    let node_addresses = vec![
        "192.168.0.101".to_string(),
        "192.168.0.102".to_string(),
        "192.168.0.103".to_string()
    ];
    let manager = NodeManager::new(node_addresses, 4);

    // Start cluster listener
    manager.start_cluster_listener(8080);

    // Discover nodes
    let available_nodes = manager.discover_nodes();

    // Create test tensor
    let tensor = MorphicTensor::void();

    // Create operations
    let operations = vec![
        QuantumOperation::H,
        QuantumOperation::X,
        QuantumOperation::CX,
        QuantumOperation::T,
        QuantumOperation::Measure,
        QuantumOperation::Z,
    ];

    // Distribute operations
    manager.distribute_operations(&operations, &tensor);

    // Schedule tasks
    let scheduler = TaskScheduler::new();
    scheduler.schedule_tasks(&operations, &available_nodes);

    // Simulate node failure
    scheduler.handle_failure("192.168.0.102");

    // Aggregate results
    let aggregator = ResultAggregator::new();
    let results = vec![0.75, 0.82, 0.68, 0.91];
    let final_result = aggregator.aggregate_results(&results);
    println!("Final aggregated result: {:.4}", final_result);

    println!("✅ Distributed orchestration tests completed!");
}
