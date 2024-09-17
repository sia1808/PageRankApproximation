// making sure it uses the function in the other file
mod pagerank;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use pagerank::random_moves;

fn read_graph(filename: &str) -> io::Result<(usize, Vec<Vec<usize>>)> {
    // Open the file and create a buffered reader
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    // Read the first line to get the number of vertices
    let vertices = lines.next().unwrap()?.parse::<usize>().unwrap();
    let mut edges = vec![Vec::new(); vertices];

    // read and make the adjacency list
    for a in lines {
        let edge = a?;
        let mut iter = edge.split_whitespace();
        let src = iter.next().unwrap().parse::<usize>().unwrap();
        let location = iter.next().unwrap().parse::<usize>().unwrap();
        edges[src].push(location);
    }

    Ok((vertices, edges))
}


fn main() {
    // Reading the graph from the file
    let graph_result = read_graph("pagerank_data.txt");
    
    // Handling the result to extract the value
    let (vertices, graph) = match graph_result {
        Ok((vertices, graph)) => (vertices, graph),
        Err(e) => {
            eprintln!("Error reading graph: {}", e);
            return;
        }
    };
    
    // Calculating PageRank values for each vertex
    let mut rng = rand::thread_rng();
    let pagerank = random_moves(vertices, &graph, &mut rng);

    // Converting into a vector of tuples sorted by the PageRank values
    let mut pagerank_vec: Vec<_> = pagerank.iter().enumerate().collect();
    pagerank_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    // Iterating over the top vertices with the highest values
    for (index, (vertex, rank)) in pagerank_vec.iter().enumerate() {
        println!(
            "vertex {}: approximate PageRank {:.4}",
            vertex, rank
        );
        // Breaking the loop if the top 5 vertices are printed already
        if index == 4 || index == pagerank_vec.len() - 1 {
            break;
        }
    }
}

// Testing to make sure it is correct
#[test]
fn test_read_graph() {
    let graph = read_graph("pagerank_data.txt");
    // Assert check
    assert_eq!(graph.len(), 3);
    assert_eq!(graph[&0], vec![1, 2]);
    assert_eq!(graph[&1], vec![2]);
    assert_eq!(graph[&2], vec![0]);
}
