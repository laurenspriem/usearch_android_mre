use rand::random;
use usearch::{Index, IndexOptions, MetricKind, ScalarKind};

fn main() {
    let mut options = IndexOptions::default();
    options.dimensions = 192; // Set the number of dimensions for vectors
    options.metric = MetricKind::Cos; // Use cosine similarity for distance measurement
    options.quantization = ScalarKind::F32; // Use 32-bit floating point numbers
    options.connectivity = 0; // zero for auto
    options.expansion_add = 0; // zero for auto
    options.expansion_search = 0; // zero for auto
    let index = Index::new(&options).expect("Failed to create index.");
    index.reserve(1000).expect("Failed to reserve capacity.");

    let start_key: u32 = 1;
    let fill: u32 = 100;
    let keys: Vec<u64> = (start_key..fill).map(|x| x as u64).collect();
    for i in &keys {
        let mut vector: Vec<f32> = (0..192).map(|_| random()).collect();
        vector[0] = *i as f32;
        index.add(*i, &vector).expect("Failed to add vector.");
    }

    let query: Vec<f32> = (0..192).map(|_| random()).collect();
    let results = index.search(&query, 3).expect("Search failed.");
    for (key, distance) in results.keys.iter().zip(results.distances.iter()) {
        println!("Key: {}, Distance: {}", key, distance);
    }
}
