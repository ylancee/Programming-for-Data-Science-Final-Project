mod data_reading;
mod separation_deg;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "euroroad.csv";
    // Build an adjacency list representation of the road network
    let adjacency_list = data_reading::build_adjacency_list_from_csv(file_path)?;

    // Calculate the maximum degree of separation in the road network graph.
    let max_degree_of_separation = separation_deg::calculate_max_degree_of_separation(&adjacency_list);
    println!("Max Degree of Separation: {}", max_degree_of_separation);

    // Calculate the average maximum degree of the road network graph.
    let average_max_degree = separation_deg::calculate_average_max_degree(&adjacency_list);
    println!("Average Max Degree: {}", average_max_degree);

    // Calculate the number of connected components in the road network graph.
    let connected_components = separation_deg::calculate_connected_components(&adjacency_list);
    println!("Number of Connected Components: {}", connected_components);

    // Calculate the average shortest path length in the road network graph.
    let average_shortest_path_length = separation_deg::calculate_average_shortest_path_length(&adjacency_list);
    println!("Average Shortest Path Length: {}", average_shortest_path_length);

    // Calculate the mean and standard deviation of separation degrees in the graph.
    let (mean, std_dev) = separation_deg::calculate_mean_and_std_dev(&adjacency_list);

    // Print the results
    println!("Mean of Separations: {}", mean);
    println!("Standard Deviation of Separations: {}", std_dev);

    // Calculate the normalized separation distribution and find the degree with the maximum percentage.
    let (normalized_separation_distribution, degree_with_max_percentage, max_percentage) = separation_deg::calculate_normalized_separation_distribution(&adjacency_list);
    println!("----------------");
    println!("Separation Distribution (degree: percentage): {:?}", normalized_separation_distribution);
    println!("----------------");
    println!("Degree with Maximum Percentage: {}, Percentage: {}", degree_with_max_percentage, max_percentage);

    Ok(())
}

// Unit test
#[cfg(test)]
mod tests {
    use super::separation_deg::calculate_normalized_separation_distribution;
    use super::separation_deg::build_sample_network;

    #[test]
    fn test_separation_distribution_sums_to_one() {
        let adjacency_list = build_sample_network();
        // Calculate the normalized separation distribution which shows how nodes are separated in the network.
        let (separation_distribution, _, _) = calculate_normalized_separation_distribution(&adjacency_list);
        let sum_of_percentages: f64 = separation_distribution.values().sum();
        assert!((sum_of_percentages - 1.0).abs() < f64::EPSILON);
    }
}

