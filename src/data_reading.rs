use csv::ReaderBuilder;
use hashbrown::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;

// This function reads a CSV file where each line represents an edge in a graph and constructs an adjacency list, which is a common way to represent graphs.
/// The graph is undirected, so an edge from `city1` to `city2` implies an edge back from `city2` to `city1`.
pub fn build_adjacency_list_from_csv(file_path: &str) -> Result<HashMap<i32, HashSet<i32>>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(File::open(file_path)?);
    let mut adjacency_list: HashMap<i32, HashSet<i32>> = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        let city1: i32 = record[0].parse()?;
        let city2: i32 = record[1].parse()?;

        adjacency_list.entry(city1).or_default().insert(city2);
        adjacency_list.entry(city2).or_default().insert(city1);
    }

    Ok(adjacency_list)
}
