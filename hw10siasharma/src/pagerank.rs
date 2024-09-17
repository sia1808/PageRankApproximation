use rand::prelude::ThreadRng;
use rand::prelude::SliceRandom;
use rand::Rng;

pub fn random_moves(vertices: usize, edges: &[Vec<usize>], rng: &mut ThreadRng) -> Vec<f64> {
    // Initializes a vector to hold the counts of visits for each vertex
    let mut counts = vec![0; vertices];
    let walks = 100;
    let steps = 100;

    // Loop through each vertex
    for x in 0..vertices {
        // Conduct a random walk 'walks' number of times
        for _ in 0..walks {
            let mut y = x;
            // Move 'steps' number of times
            for _ in 0..steps {
                let next_nodes = &edges[y];
                // If there are no outgoing edges or with a probability of 10% then jump to a random vertex
                if next_nodes.is_empty() || rng.gen_bool(0.1) {
                    y = rng.gen_range(0..vertices);
                //else choose the next vertex randomly from neighbors
                } else {
                    y = *next_nodes.choose(rng).unwrap();
                }
            }
            counts[y] += 1;
        }
    }
    // Calculate PageRank values by dividing counts by total walks
    let total_walks = walks * vertices;
    counts.into_iter().map(|count| count as f64 / total_walks as f64).collect()
}