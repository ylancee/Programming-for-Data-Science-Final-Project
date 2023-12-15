use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;

// Perform a Breadth-First Search (BFS) to find the shortest paths from a starting node to all other nodes.
// Returns a HashMap where the keys are node identifiers and the values are the shortest distances from the start node.
pub fn bfs(adj_list: &HashMap<i32, HashSet<i32>>, start_node: i32) -> HashMap<i32, i32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();

    // Start by adding the start node to the visited set and queue.
    visited.insert(start_node);
    queue.push_back((start_node, 0));

    // Continue until there are no more nodes to visit.
    while let Some((current_node, distance)) = queue.pop_front() {
        distances.insert(current_node, distance);

        // Look at all the neighbors of the current node.
        for &neighbor in &adj_list[&current_node] {
            if visited.insert(neighbor) {
                queue.push_back((neighbor, distance + 1));
            }
        }
    }

    distances
}

// This is the maximum shortest path length from any node to any other node.
pub fn calculate_max_degree_of_separation(adjacency_list: &HashMap<i32, HashSet<i32>>) -> i32 {
    let max_degrees = adjacency_list.keys()
        .map(|&city| {
            let distances = bfs(&adjacency_list, city);
            *distances.values().max().unwrap_or(&0)
        })
        .collect::<Vec<_>>();

    *max_degrees.iter().max().unwrap_or(&0)
}

// Calculate the average of the maximum degree of separation for each node.
pub fn calculate_average_max_degree(adjacency_list: &HashMap<i32, HashSet<i32>>) -> f64 {
    let max_degrees = adjacency_list.keys()
        .map(|&city| {
            let distances = bfs(&adjacency_list, city);
            *distances.values().max().unwrap_or(&0)
        })
        .collect::<Vec<_>>();

    max_degrees.iter().sum::<i32>() as f64 / max_degrees.len() as f64
}

// Connected components are groups of nodes where each node is reachable from any other node in the same group.
pub fn calculate_connected_components(adjacency_list: &HashMap<i32, HashSet<i32>>) -> usize {
    // Use BFS to find the maximum degree of separation for each node.
    // We then collect unique maximum degrees, which corresponds to separate connected components.
    adjacency_list.keys()
        .map(|&city| {
            let distances = bfs(&adjacency_list, city);
            *distances.values().max().unwrap_or(&0)
        })
        .collect::<HashSet<_>>()
        .len()
}

// This is the average number of edges on the shortest path between pairs of nodes.
pub fn calculate_average_shortest_path_length(adjacency_list: &HashMap<i32, HashSet<i32>>) -> f64 {
    // Use BFS to calculate the total length of shortest paths and the number of such paths.
    let (total_length, total_paths) = adjacency_list.keys().fold((0, 0), |(total_length, total_paths), &city| {
        let distances = bfs(&adjacency_list, city);
        distances.values().fold((total_length, total_paths), |(length, paths), &distance| {
            if distance > 0 {
                (length + distance, paths + 1)
            } else {
                (length, paths)
            }
        })
    });

    total_length as f64 / total_paths as f64
}

// This returns a distribution of the shortest path lengths between nodes, the degree with the maximum percentage, and the corresponding percentage.
pub fn calculate_normalized_separation_distribution(adjacency_list: &HashMap<i32, HashSet<i32>>) -> (HashMap<i32, f64>, i32, f64) {
    let mut total_paths = 0;
    let mut separation_distribution: HashMap<i32, i32> = HashMap::new();

    // Use BFS to find all path lengths and record the frequency of each path length.
    for &city in adjacency_list.keys() {
        let distances = bfs(&adjacency_list, city);
        for &length in distances.values() {
            if length > 0 { // Exclude the path to itself
                *separation_distribution.entry(length).or_insert(0) += 1;
                total_paths += 1;
            }
        }
    }

    // Normalize the separation distribution so it sums to 1.
    let normalized_separation_distribution: HashMap<i32, f64> = separation_distribution
        .iter()
        .map(|(&degree, &count)| (degree, count as f64 / total_paths as f64))
        .collect();

    // Find the path length that occurs most frequently.
    let (&degree_with_max_percentage, &max_percentage) = normalized_separation_distribution
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap_or((&0, &0.0));

    (normalized_separation_distribution, degree_with_max_percentage, max_percentage)
}

// This gives us an idea of the graph's connectivity and its variance.
pub fn calculate_mean_and_std_dev(adjacency_list: &HashMap<i32, HashSet<i32>>) -> (f64, f64) {
    let mut all_distances = Vec::new();

    // Collect all distances between nodes
    for &city in adjacency_list.keys() {
        let distances = bfs(adjacency_list, city);
        for &distance in distances.values() {
            if distance > 0 { // Exclude the distance to the node itself
                all_distances.push(distance);
            }
        }
    }

    // Calculate the mean of all path lengths.
    let mean: f64 = all_distances.iter().sum::<i32>() as f64 / all_distances.len() as f64;
    
    // Calculate the variance and then the standard deviation to measure how much the path lengths vary.
    let variance: f64 = all_distances.iter()
        .map(|&distance| {
            let diff = distance as f64 - mean;
            diff * diff
        })
        .sum::<f64>() / all_distances.len() as f64;

    // Calculate the standard deviation
    let std_dev = variance.sqrt();

    (mean, std_dev)
}

// Create a sample graph for testing purposes.
#[cfg(test)]
pub fn build_sample_network() -> HashMap<i32, HashSet<i32>> {
    let mut adjacency_list: HashMap<i32, HashSet<i32>> = HashMap::new();

    // Define a small graph manually by adding edges. This graph is a simple triangle connecting nodes 1, 2, and 3.
    // 1 - 2 - 3
    adjacency_list.entry(1).or_default().insert(2);
    adjacency_list.entry(2).or_default().insert(1);
    adjacency_list.entry(2).or_default().insert(3);
    adjacency_list.entry(3).or_default().insert(2);

    adjacency_list
}